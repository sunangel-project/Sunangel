package main

import (
	"encoding/json"
	"log"
	"strconv"
	"sunangel/horizon/messages"
	"sunangel/messaging"
	"time"

	"github.com/nats-io/nats.go"
)

const (
	HOR_STORE_NAME  = "horizons"
	COMP_STORE_NAME = "horizons-in-computation"

	IN_Q  = "SPOTS.get-horizon"
	GROUP = "horizon-get-service"

	REQ_OUT_Q      = "SPOTS.compute-horizon"
	RES_OUT_STREAM = "HORIZONS"
	RES_OUT_Q      = RES_OUT_STREAM + ".sunsets"
	ERR_STREAM     = "ERRORS"

	REQUEUE_SECONDS = 10
)

type Communications struct {
	js     nats.JetStreamContext
	kvHor  nats.KeyValue
	kvComp nats.KeyValue
}

func main() {
	nc := messaging.Connect()
	defer nc.Close()
	js := messaging.JetStream(nc)

	kvHor := messaging.ConnectOrCreateKV(js, HOR_STORE_NAME)
	kvComp := messaging.ConnectOrCreateKV(js, COMP_STORE_NAME)

	coms := &Communications{js, kvHor, kvComp}

	err := messaging.SetupStreams(js, []string{RES_OUT_STREAM, ERR_STREAM})
	if err != nil {
		panic(err)
	}

	_, err = js.QueueSubscribe(IN_Q, GROUP, func(msg *nats.Msg) {
		err := handleMessage(msg, coms)
		if err != nil {
			log.Printf("error while handling message: %s\nmessage: %v", err, msg)
		}
	})
	if err != nil {
		panic(err)
	}
}

func handleMessage(msg *nats.Msg, coms *Communications) error {
	var req messages.HorizonRequest
	if err := json.Unmarshal(msg.Data, &req); err != nil {
		return err
	}

	key := messages.HorizonKey(req.Spot.Loc, 500)
	if _, err := coms.kvHor.Get(key); err != nil {
		if !messages.IsKeyDoesntExistsError(err) {
			return err
		}

		handleMissingHorizon(msg, key, coms)
	} else {
		forwardHorizonKey(msg, key, coms)
	}

	if err := msg.Ack(); err != nil {
		return err
	}

	return nil
}

func handleMissingHorizon(
	msg *nats.Msg,
	key string,
	coms *Communications,
) error {
	isInCompute, err := isHorizonInCompute(key, coms)
	if err != nil {
		return err
	}

	if isInCompute {
		err = requeueGetRequest(msg, key, coms)
	} else {
		_, err = coms.js.Publish(REQ_OUT_Q, msg.Data)
	}
	return err
}

func isHorizonInCompute(
	key string,
	coms *Communications,
) (bool, error) {
	isInComputeEntry, err := coms.kvComp.Get(key)
	if err != nil {
		if messages.IsKeyDoesntExistsError(err) {
			return false, nil
		}

		return false, err
	}

	return decodeIsIncomputeEntry(isInComputeEntry)
}

func decodeIsIncomputeEntry(
	entry nats.KeyValueEntry,
) (bool, error) {
	isInCompute, err := strconv.ParseBool(
		string(entry.Value()),
	)
	return isInCompute, err
}

func requeueGetRequest(
	msg *nats.Msg,
	key string,
	coms *Communications,
) error {
	watch, err := coms.kvComp.Watch(key)
	if err != nil {
		return err
	}
	timer := time.NewTimer(REQUEUE_SECONDS * time.Second)

	requeue := func() error {
		_, err := coms.js.Publish(IN_Q, msg.Data)
		return err
	}

	for {
		select {
		case <-timer.C:
			return requeue()
		case update := <-watch.Updates():
			isInCompute, err := decodeIsIncomputeEntry(update)
			if err != nil {
				return err
			}

			if !isInCompute {
				return forwardHorizonKey(msg, key, coms)
			}
		}
	}
}

func forwardHorizonKey(
	msg *nats.Msg,
	key string,
	coms *Communications,
) error {
	var msgData messages.HorizonResult
	if err := json.Unmarshal(msg.Data, &msgData); err != nil {
		return err
	}

	msgData.Horizon = key

	msgPayload, err := json.Marshal(msgData)
	if err != nil {
		return err
	}

	if _, err := coms.js.Publish(RES_OUT_Q, msgPayload); err != nil {
		return err
	}

	err = msg.Ack()
	return err
}
