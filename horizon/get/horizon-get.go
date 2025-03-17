package main

import (
	"context"
	"encoding/json"
	"log"
	"sync"
	"time"

	"github.com/nats-io/nats.go/jetstream"
	"github.com/sirupsen/logrus"

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
	logrus.SetLevel(logrus.TraceLevel)
	logrus.Infof("Starting up (version %s)", common.BACKEND_VERSION)
	defer logrus.Info("Shutting down")

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
		Durable:        GROUP,
		AckWait:        (REQUEUE_SECONDS + 10) * time.Second,
		FilterSubjects: []string{IN_Q},
	}
	cons, err := messaging.ConnectOrCreateConsumer(ctx, stream, GROUP, consConfig)
	if err != nil {
		panic(err)
	}
	logrus.Infof("Setup complete, listening to %s", IN_Q)

	_, err = cons.Consume(func(msg jetstream.Msg) {
		if err := handleMessage(msg, coms); err != nil {
			logrus.WithError(err).Errorf(
				"Error while handling message: %v",
				string(msg.Data()),
			)
			_ = msg.Nak()
		}
	})
	if err != nil {
		panic(err)
	}

	// avoid shutdown
	var wg sync.WaitGroup
	wg.Add(1)
	wg.Wait()
}

func handleMessage(msg jetstream.Msg, coms *common.Communications) error {
	logrus.Trace("Unmarshaling the request")
	var req messages.HorizonRequest
	if err := json.Unmarshal(msg.Data(), &req); err != nil {
		return err
	}

	err := handleRequest(msg, req, coms)

	if err != nil {
		err := common.SendError(string(msg.Data()), err, req.RequestId, GROUP, coms)
		if err != nil {
			log.Printf("Could not send out error: %s", err)
		}
	}

	return err
}

func handleRequest(
	msg jetstream.Msg,
	req messages.HorizonRequest,
	coms *common.Communications,
) error {
	logrus.Trace("Handling the request")
	key := common.HorizonKey(req.Spot.Loc, 500)

	logrus.Tracef("Checking for horizon key %s - belonging to location %#v", key, req.Spot.Loc)
	if _, err := coms.KvHor.Get(coms.Ctx, key); err != nil {
		logrus.Trace("Received errror when checking for horizon")
		if !common.IsKeyDoesntExistsError(err) {
			logrus.WithError(err).Trace("Error was not a KeyDoesntExistError")
			return err
		}

		err = handleMissingHorizon(msg, req.RequestId, key, coms)
		if err != nil {
			return err
		}
	} else {
		logrus.Trace("Forwarding the horizon key")
		if err := common.ForwardHorizonKey(msg, key, coms); err != nil {
			return err
		}

		err = msg.Ack()
		if err != nil {
			return err
		}
	}

	return nil
}

func handleMissingHorizon(
	msg jetstream.Msg,
	requestId string,
	key string,
	coms *common.Communications,
) error {
	logrus.Tracef("Handling missing horizon %s", key)
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

		logrus.Tracef("Queueing request to compute horizon %s", key)
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
		logrus.WithError(err).Errorf(
			"Error while handling message: %v",
			string(msg.Data()),
		)

		if sendErr := common.SendError(
			string(msg.Data()),
			err,
			requestId,
			GROUP,
			coms,
		); sendErr != nil {
			logrus.WithError(sendErr).Errorf("Could not send out error '%s'", err)
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
	for nil != <-updates {
	} // nil signals the end of historical data
	// https://pkg.go.dev/github.com/nats-io/nats.go/jetstream#readme-watching-for-changes-on-a-bucket

	timer := time.NewTimer(REQUEUE_SECONDS * time.Second)

	for {
		select {
		case <-timer.C:
			return msg.Nak()
		case update := <-updates:
			if nil == update {
				logrus.Warn("Received nil update in waiting loop. This should not happen, please review")
				break
			}
			logrus.WithFields(logrus.Fields{
				"operation": update.Operation(),
				"value":     string(update.Value()),
			}).Trace("Received update")

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
