package main

import (
	"context"
	"encoding/json"
	"fmt"
	"sync"
	"time"

	"github.com/joho/godotenv"
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
	err := godotenv.Load()
	if err != nil {
		panic(err)
	}

	err = messaging.SetLogLevel()
	if err != nil {
		panic(err)
	}

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
			if err := msg.Nak(); err != nil {
				logrus.
					WithField("request", string(msg.Data())).
					WithError(err).
					Error("Could not nak request message")
			}
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
	logrus.WithField("request", string(msg.Data())).Trace("Parsing the request")
	var req messages.HorizonRequest
	if err := json.Unmarshal(msg.Data(), &req); err != nil {
		return fmt.Errorf("could not parse request '%s': %w", string(msg.Data()), err)
	}
	logrus.
		WithFields(logrus.Fields{
			"id":   req.RequestId,
			"spot": req.Spot,
		}).
		Info("Received request")

	err := handleRequest(msg, req, coms)
	if err != nil {
		sendError := common.SendError(string(msg.Data()), err, req.RequestId, GROUP, coms)
		if sendError != nil {
			logrus.
				WithField("id", req.RequestId).
				WithError(sendError).
				Errorf("Could not send out error '%s'", err)
		}
	}

	return nil
}

func handleRequest(
	msg jetstream.Msg,
	req messages.HorizonRequest,
	coms *common.Communications,
) error {
	key := common.HorizonKey(req.Spot.Loc, 500)

	logger := logrus.
		WithFields(logrus.Fields{
			"request": req.RequestId,
			"spot":    req.Spot,
			"key":     key,
		})
	logger.Trace("Checking for horizon key")
	if _, err := coms.KvHor.Get(coms.Ctx, key); err != nil {
		logger.WithError(err).Trace("Received error when checking for key in key value store")
		if !common.IsKeyDoesntExistsError(err) {
			return fmt.Errorf("received unexpected error when checking whether horizon is being computed: %w", err)
		}

		if err := handleMissingHorizon(msg, req.RequestId, key, coms, logger); err != nil {
			return fmt.Errorf("error while handling the missing horizon: %w", err)
		}
	} else {
		logger.Trace("Horizon exists in cache")
		if err := common.ForwardHorizonKey(msg, key, coms); err != nil {
			return fmt.Errorf("could not forward horizon key: %w", err)
		}

		if err := msg.Ack(); err != nil {
			return fmt.Errorf("could not acknowledge request message: %w", err)
		}
	}

	return nil
}

func handleMissingHorizon(
	msg jetstream.Msg,
	requestId string,
	key string,
	coms *common.Communications,
	logger *logrus.Entry,
) error {
	logger.Trace("Checking whether the horizon is in compute")
	isInCompute, err := common.IsHorizonInCompute(key, coms)
	if err != nil {
		return err
	}

	if isInCompute {
		logger.Trace("The horizon is already being computed")
		go requeueGetRequestAndLog(msg, requestId, key, coms, logger)
	} else {
		logger.Trace("The horizon is not being computed yet, marking as in compute")
		if err := common.SetHorizonInCompute(key, true, coms); err != nil {
			return fmt.Errorf("could not mark horizon as in compute: %w", err)
		}

		logger.Trace("Sending a request to compute the horizon")
		if _, err := coms.Js.Publish(
			coms.Ctx,
			common.REQ_COMP_Q,
			msg.Data(),
		); err != nil {
			return err
		}

		if err := msg.Ack(); err != nil {
			return fmt.Errorf("could not acknowledge request message: %w", err)
		}
	}
	return nil
}

func requeueGetRequestAndLog(
	msg jetstream.Msg,
	requestId string,
	key string,
	coms *common.Communications,
	logger *logrus.Entry,
) {
	if err := requeueGetRequest(msg, key, coms, logger); err != nil {
		logger.WithError(err).Error("Error while handling message")

		if sendErr := common.SendError(
			string(msg.Data()),
			err,
			requestId,
			GROUP,
			coms,
		); sendErr != nil {
			logger.WithError(sendErr).Errorf("Could not send out error '%s'", err)
		}

		if err := msg.Nak(); err != nil {
			logger.WithError(err).Error("Could not nak request message")
		}
	}
}

func requeueGetRequest(
	msg jetstream.Msg,
	key string,
	coms *common.Communications,
	logger *logrus.Entry,
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
			if err := msg.Nak(); err != nil {
				logger.WithError(err).Error("Could not nak request message")
			}
			return nil
		case update := <-updates:
			if nil == update {
				logger.Warn("Received nil update in waiting loop. This should not happen, please review")
				break
			}

			isInCompute, err := common.DecodeIsIncomputeEntry(update)
			if err != nil {
				return err
			}

			if !isInCompute {
				if err := common.ForwardHorizonKey(msg, key, coms); err != nil {
					return fmt.Errorf("could not send out the horizon key: %w", err)
				}

				if err := msg.Ack(); err != nil {
					return fmt.Errorf("could not acknowledge the request message: %w", err)
				}

				return nil
			}
		}
	}
}
