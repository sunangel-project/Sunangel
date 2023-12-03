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
	GROUP = "horizon-get-service"

	IN_Q = common.REQ_GET_Q

	REQUEUE_SECONDS = 10
)

func main() {
	nc := messaging.Connect()
	defer nc.Close()

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	js := messaging.JetStream(nc)

	kvHor := messaging.ConnectOrCreateKV(ctx, js, common.HOR_STORE_NAME)
	kvComp := messaging.ConnectOrCreateKV(ctx, js, common.COMP_STORE_NAME)

	coms := &common.Communications{
		Ctx:    ctx,
		Js:     js,
		KvHor:  kvHor,
		KvComp: kvComp,
	}

	if err := messaging.SetupStreams(ctx, js, []string{
		common.RES_OUT_STREAM,
		common.ERR_STREAM,
	}); err != nil {
		panic(err)
	}

	stream, err := js.Stream(ctx, common.SPOT_STREAM)
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
	var err error
	key := common.HorizonKey(req.Spot.Loc, 500)
	if _, err := coms.KvHor.Get(coms.Ctx, key); err != nil {
		if !common.IsKeyDoesntExistsError(err) {
			return err
		}

		err = handleMissingHorizon(msg, req.RequestId, key, coms)
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
	requestId string,
	key string,
	coms *common.Communications,
) error {
	isInCompute, err := common.IsHorizonInCompute(key, coms)
	if err != nil {
		return err
	}

	if isInCompute {
		go requeueGetRequestAndLog(msg, requestId, key, coms)
	} else {
		if err := common.SetHorizonInCompute(key, true, coms); err != nil {
			return err
		}

		if _, err := coms.Js.Publish(
			coms.Ctx,
			common.REQ_COMP_Q,
			msg.Data(),
		); err != nil {
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
	requestId string,
	key string,
	coms *common.Communications,
) {
	if err := requeueGetRequest(msg, key, coms); err != nil {
		log.Printf(
			"error while handling message: %s\nmessage: %v",
			err, string(msg.Data()),
		)

		if err := common.SendError(
			string(msg.Data()),
			err,
			requestId,
			GROUP,
			coms,
		); err != nil {
			log.Printf("could not send out error: %s", err)
		}

		_ = msg.Nak() // Ignoring error
	}
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

	for {
		select {
		case <-timer.C:
			return msg.Nak()
		case update := <-updates:
			isInCompute, err := common.DecodeIsIncomputeEntry(update)
			if err != nil {
				return err
			}

			if !isInCompute {
				if err := common.ForwardHorizonKey(msg, key, coms); err != nil {
					return err
				}
				return msg.Ack()
			}
		}
	}
}
