package main

import (
	"context"
	"encoding/json"
	"log"
	"time"

	"github.com/nats-io/nats.go/jetstream"

	"sunangel/horizon/common"
	"sunangel/horizon/messages"
	"sunangel/messaging"
)

const (
	HOR_STORE_NAME  = "horizons"
	COMP_STORE_NAME = "horizons-in-computation"

	GROUP = "horizon-get-service"

	IN_STREAM  = "SPOTS"
	IN_SUBJECT = "get-horizon"
	IN_Q       = IN_STREAM + "." + IN_SUBJECT

	REQ_OUT_Q = "SPOTS.compute-horizon"

	ERR_STREAM = "ERRORS"
	ERR_Q      = ERR_STREAM + "." + GROUP

	REQUEUE_SECONDS = 10
)

func main() {
	nc := messaging.Connect()
	defer nc.Close()

	ctx, cancel := context.WithTimeout(context.Background(), 30*time.Second)
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
		AckWait:        (REQUEUE_SECONDS + 10) * time.Second,
		FilterSubjects: []string{IN_SUBJECT},
	}
	cons, err := messaging.ConnectOrCreateConsumer(ctx, stream, GROUP, consConfig)
	if err != nil {
		panic(err)
	}

	_, err = cons.Consume(func(msg jetstream.Msg) {
		err := handleMessage(msg, coms)
		if err != nil {
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

func handleMessage(msg jetstream.Msg, coms *common.Communications) error {
	var req messages.HorizonRequest
	if err := json.Unmarshal(msg.Data(), &req); err != nil {
		return err
	}

	log.Print("decoded request")

	var err error
	key := common.HorizonKey(req.Spot.Loc, 500)
	if _, err := coms.KvHor.Get(coms.Ctx, key); err != nil {
		if !common.IsKeyDoesntExistsError(err) {
			return err
		}

		err = handleMissingHorizon(msg, key, coms)
	} else {
		err = common.ForwardHorizonKey(msg, key, coms)
	}
	if err != nil {
		return err
	}

	// return msg.Ack()
	return nil
}

func handleMissingHorizon(
	msg jetstream.Msg,
	key string,
	coms *common.Communications,
) error {
	log.Print("handle missing hor")
	isInCompute, err := common.IsHorizonInCompute(key, coms)
	if err != nil {
		return err
	}
	log.Printf("is in compute: %t", isInCompute)

	if isInCompute {
		err = requeueGetRequest(msg, key, coms)
	} else {
		if err := common.SetHorizonInCompute(key, true, coms); err != nil {
			return err
		}

		_, err = coms.Js.Publish(coms.Ctx, REQ_OUT_Q, msg.Data())
	}
	return err
}

func requeueGetRequest(
	msg jetstream.Msg,
	key string,
	coms *common.Communications,
) error {
	watch, err := coms.KvComp.Watch(coms.Ctx, key)
	if err != nil {
		return err
	}
	updates := watch.Updates()
	for <-updates != nil {
	}

	timer := time.NewTimer(REQUEUE_SECONDS * time.Second)

	requeue := func() error {
		_, err := coms.Js.Publish(
			coms.Ctx,
			IN_Q,
			msg.Data(),
		)
		return err
	}

	for {
		select {
		case <-timer.C:

			log.Print("times up")
			return requeue()
		case update := <-updates:
			log.Printf("update received: %v", update)

			isInCompute, err := common.DecodeIsIncomputeEntry(update)
			if err != nil {
				return err
			}

			if !isInCompute {
				return common.ForwardHorizonKey(msg, key, coms)
			}
		}
	}
}
