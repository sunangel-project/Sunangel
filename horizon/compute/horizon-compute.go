package main

import (
	"context"
	"encoding/json"
	"log"
	"time"

	"github.com/nats-io/nats.go/jetstream"
	"github.com/sunangel-project/horizon"
	"github.com/sunangel-project/horizon/location"

	"sunangel/horizon/common"
	"sunangel/horizon/messages"
	"sunangel/messaging"
)

const (
	HOR_STORE_NAME  = "horizons"
	COMP_STORE_NAME = "horizons-in-computation"

	GROUP = "horizon-compute-service"

	IN_STREAM  = "SPOTS"
	IN_SUBJECT = "compute-horizon"
	IN_Q       = IN_STREAM + ".compute-horizon"

	ERR_STREAM = "ERRORS"
	ERR_Q      = ERR_STREAM + "." + GROUP
)

func main() {
	nc := messaging.Connect()
	defer nc.Close()

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	js := messaging.JetStream(nc)

	kvHor := messaging.ConnectOrCreateKV(ctx, js, HOR_STORE_NAME)
	kvComp := messaging.ConnectOrCreateKV(ctx, js, COMP_STORE_NAME)

	coms := &common.Communications{
		Ctx:    ctx,
		Js:     js,
		KvHor:  kvHor,
		KvComp: kvComp,
	}

	if err := messaging.SetupStreams(ctx, js, []string{
		common.RES_OUT_STREAM,
		ERR_STREAM,
	}); err != nil {
		panic(err)
	}

	stream, err := js.Stream(ctx, IN_STREAM)
	if err != nil {
		panic(err)
	}

	consConfig := jetstream.ConsumerConfig{
		Name:           GROUP,
		FilterSubjects: []string{IN_Q},
	}
	cons, err := messaging.ConnectOrCreateConsumer(ctx, stream, GROUP, consConfig)
	if err != nil {
		panic(err)
	}
	log.Print("Setup complete, listening to " + IN_Q)

	_, err = cons.Consume(func(msg jetstream.Msg) {
		if err := handleMessage(msg, coms); err != nil {
			log.Printf(
				"error while handling message: %s\nmessage: %v",
				err, string(msg.Data()),
			)
		}
	})
	if err != nil {
		panic(err)
	}

	for { // avoid shutdown
		time.Sleep(time.Hour)
	}
}

func handleMessage(
	msg jetstream.Msg,
	coms *common.Communications,
) error {
	log.Printf("received message: %s", string(msg.Data()))

	var req messages.HorizonRequest
	if err := json.Unmarshal(msg.Data(), &req); err != nil {
		return err
	}

	err := handleRequest(msg, req, coms)

	if err != nil {
		err := common.SendError(string(msg.Data()), err, req.RequestId, GROUP, coms)
		if err != nil {
			log.Printf("could not send out error: %s", err)
		}
	}

	return err
}

func handleRequest(
	msg jetstream.Msg,
	req messages.HorizonRequest,
	coms *common.Communications,
) error {
	radius := 500
	key := common.HorizonKey(req.Spot.Loc, radius)

	loc := location.Location{
		Latitude:  req.Spot.Loc.Lat,
		Longitude: req.Spot.Loc.Lon,
	}
	log.Printf(
		"Computing horizon for spot %s\ncoordinates: %v\nradius: %d",
		key, loc, radius,
	)
	hor := horizon.NewHorizon(&loc, radius)

	if _, err := coms.KvHor.Put(coms.Ctx, key, hor.AltitudeToBytes()); err != nil {
		return err
	}

	if err := common.SetHorizonInCompute(key, false, coms); err != nil {
		return err
	}

	if err := common.ForwardHorizonKey(msg, key, coms); err != nil {
		return err
	}

	return msg.Ack()
}
