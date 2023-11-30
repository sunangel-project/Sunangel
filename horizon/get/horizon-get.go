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

	IN_STREAM = "SPOTS"
	IN_Q      = IN_STREAM + ".get-horizon"

	REQ_OUT_Q = "SPOTS.compute-horizon"

	ERR_STREAM = "ERRORS"
	ERR_Q      = ERR_STREAM + "." + GROUP

	REQUEUE_SECONDS = 10
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
		AckWait:        (REQUEUE_SECONDS + 10) * time.Second,
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
			msg.Nak()
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

	var err error
	key := common.HorizonKey(req.Spot.Loc, 500)
	if _, err := coms.KvHor.Get(coms.Ctx, key); err != nil {
		if !common.IsKeyDoesntExistsError(err) {
			return err
		}

		err = handleMissingHorizon(msg, key, coms)
	} else {
		if err := common.ForwardHorizonKey(msg, key, coms); err != nil {
			return err
		}

		err = msg.Ack()
	}
	return err
}

func handleMissingHorizon(
	msg jetstream.Msg,
	key string,
	coms *common.Communications,
) error {
	isInCompute, err := common.IsHorizonInCompute(key, coms)
	if err != nil {
		return err
	}

	if isInCompute {
		go requeueGetRequestAndLog(msg, key, coms)
	} else {
		if err := common.SetHorizonInCompute(key, true, coms); err != nil {
			return err
		}

		if _, err := coms.Js.Publish(coms.Ctx, REQ_OUT_Q, msg.Data()); err != nil {
			return err
		}

		if err := msg.Ack(); err != nil {
			return err
		}
	}
	return nil
}

func requeueGetRequestAndLog(
	msg jetstream.Msg,
	key string,
	coms *common.Communications,
) {
	if err := requeueGetRequest(msg, key, coms); err != nil {
		log.Printf(
			"error while handling message: %s\nmessage: %v",
			err, string(msg.Data()),
		)
		_ = msg.Nak() // Ignoring error
	}
}

func requeueGetRequest(
	msg jetstream.Msg,
	key string,
	coms *common.Communications,
) error {
	log.Printf("horizon %s is in compute", key)
	watch, err := coms.KvComp.Watch(coms.Ctx, key)
	if err != nil {
		return err
	}
	updates := watch.Updates()
	for <-updates != nil {
	}

	timer := time.NewTimer(REQUEUE_SECONDS * time.Second)

loop:
	for {
		select {
		case <-timer.C:
			log.Printf("horizon %s: timer", key)
			if _, err := coms.Js.Publish(
				coms.Ctx,
				IN_Q,
				msg.Data(),
			); err != nil {
				return err
			}
			break loop
		case update := <-updates:
			log.Printf("horizon %s: update %v", key, update)
			isInCompute, err := common.DecodeIsIncomputeEntry(update)
			if err != nil {
				return err
			}

			if !isInCompute {
				if err := common.ForwardHorizonKey(msg, key, coms); err != nil {
					return err
				}
				break loop
			}
		}
	}
	return msg.Ack()
}
