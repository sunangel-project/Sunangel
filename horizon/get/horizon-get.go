package main

import (
	"encoding/json"
	"log"
	"time"

	"github.com/nats-io/nats.go"

	"sunangel/horizon/common"
	"sunangel/horizon/messages"
	"sunangel/messaging"
)

const (
	HOR_STORE_NAME  = "horizons"
	COMP_STORE_NAME = "horizons-in-computation"

	IN_Q  = "SPOTS.get-horizon"
	GROUP = "horizon-get-service"

	REQ_OUT_Q = "SPOTS.compute-horizon"

	ERR_STREAM = "ERRORS"
	ERR_Q      = ERR_STREAM + "." + GROUP

	REQUEUE_SECONDS = 10
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

func handleMessage(msg *nats.Msg, coms *common.Communications) error {
	var req messages.HorizonRequest
	if err := json.Unmarshal(msg.Data, &req); err != nil {
		return err
	}

	log.Print("decoded request")

	var err error
	key := common.HorizonKey(req.Spot.Loc, 500)
	if _, err := coms.KvHor.Get(key); err != nil {
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
	msg *nats.Msg,
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

		_, err = coms.Js.Publish(REQ_OUT_Q, msg.Data)
	}
	return err
}

func requeueGetRequest(
	msg *nats.Msg,
	key string,
	coms *common.Communications,
) error {
	watch, err := coms.KvComp.Watch(key)
	if err != nil {
		return err
	}
	timer := time.NewTimer(REQUEUE_SECONDS * time.Second)

	requeue := func() error {
		_, err := coms.Js.Publish(IN_Q, msg.Data)
		return err
	}

	for {
		select {
		case <-timer.C:

			log.Print("times up")
			return requeue()
		case update := <-watch.Updates():
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
