package main

import (
	"bytes"
	"encoding/json"
	"errors"
	"log"
	"sunangel/horizon/messages"
	"sunangel/messaging"

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
	dec := json.NewDecoder(bytes.NewReader(msg.Data))

	var req messages.HorizonRequest
	err := dec.Decode(&req)
	if err != nil {
		return err
	}

	key := messages.HorizonKey(req.Spot.Loc, 500)
	if _, err := coms.kvHor.Get(key); err != nil {
		if !messages.IsKeyDoesntExistsError(err) {
			return err
		}

		// TODO: check kvComp, forward or requeue request
	} else {
		// TODO: send key to results
	}

	if err := msg.Ack(); err != nil {
		return err
	}

	return nil
}
