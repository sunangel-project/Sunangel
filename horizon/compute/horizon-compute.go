package main

import (
	"encoding/json"
	"log"
	"time"

	"github.com/nats-io/nats.go"
	"github.com/sunangel-project/horizon"
	"github.com/sunangel-project/horizon/location"

	"sunangel/horizon/common"
	"sunangel/horizon/messages"
	"sunangel/messaging"
)

const (
	HOR_STORE_NAME  = "horizons"
	COMP_STORE_NAME = "horizons-in-computation"

	IN_Q  = "SPOTS.compute-horizon"
	GROUP = "horizon-compute-service"

	ERR_STREAM = "ERRORS"
	ERR_Q      = ERR_STREAM + "." + GROUP
)

func main() {
	nc := messaging.Connect()
	defer nc.Close()
	js := messaging.JetStream(nc)

	kvHor := messaging.ConnectOrCreateKV(js, HOR_STORE_NAME)
	kvComp := messaging.ConnectOrCreateKV(js, COMP_STORE_NAME)

	coms := &common.Communications{
		Js:     js,
		KvHor:  kvHor,
		KvComp: kvComp,
	}

	if err := messaging.SetupStreams(js, []string{
		common.RES_OUT_STREAM,
		ERR_STREAM,
	}); err != nil {
		panic(err)
	}

	_, err := js.QueueSubscribe(IN_Q, GROUP, func(msg *nats.Msg) {
		err := handleMessage(msg, coms)
		if err != nil {
			log.Printf(
				"error while handling message: %s\nmessage: %v",
				err, string(msg.Data),
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
	msg *nats.Msg,
	coms *common.Communications,
) error {
	log.Printf("received message: %s", string(msg.Data))

	var req messages.HorizonRequest
	if err := json.Unmarshal(msg.Data, &req); err != nil {
		return err
	}

	radius := 500
	key := common.HorizonKey(req.Spot.Loc, radius)

	common.SetHorizonInCompute(key, true, coms)

	loc := location.Location{
		Latitude:  req.Spot.Loc.Lat,
		Longitude: req.Spot.Loc.Lon,
	}
	log.Printf(
		"Computing horizon for spot %s\ncoordinates: %v\nradius: %d",
		key, loc, radius,
	)
	hor := horizon.NewHorizon(&loc, radius)

	if _, err := coms.KvHor.Put(key, hor.AltitudeToBytes()); err != nil {
		return err
	}

	common.SetHorizonInCompute(key, false, coms)

	if err := common.ForwardHorizonKey(msg, key, coms); err != nil {
		return err
	}

	//return msg.Ack()
	return nil
}
