package main

import (
	"context"
	"encoding/json"
	"fmt"
	"sync"

	"github.com/joho/godotenv"
	"github.com/nats-io/nats.go/jetstream"
	"github.com/sirupsen/logrus"
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

	ctx := context.Background()
	coms, cons, err := setupMessaging(ctx)
	if err != nil {
		panic(err)
	}
	defer coms.Close()

	logrus.Infof("Setup complete, listening to %s", IN_Q)

	_, err = cons.Consume(func(msg jetstream.Msg) {
		logger := logrus.WithField(
			"request",
			string(msg.Data()),
		)

		if err := handleMessage(msg, coms, logger); err != nil {
			logger = logger.WithError(err)
			logger.Error("Error while handling message")
			messaging.LoggedNak(msg, logger)
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

func setupMessaging(ctx context.Context) (*common.Communications, jetstream.Consumer, error) {
	nc := messaging.Connect()

	// ctx, cancel := context.WithCancel(context.Background())

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
		return nil, nil, err
	}

	stream, err := js.Stream(ctx, IN_STREAM)
	if err != nil {
		return nil, nil, fmt.Errorf("could not connect to spot stream: %w", err)
	}

	consConfig := jetstream.ConsumerConfig{
		Durable:        GROUP,
		FilterSubjects: []string{IN_Q},
	}
	cons, err := messaging.ConnectOrCreateConsumer(ctx, stream, GROUP, consConfig)
	if err != nil {
		return nil, nil, err
	}

	return coms, cons, nil
}

func handleMessage(
	msg jetstream.Msg,
	coms *common.Communications,
	logger *logrus.Entry,
) error {
	logger.Trace("Parsing the request")
	var req messages.HorizonRequest
	if err := json.Unmarshal(msg.Data(), &req); err != nil {
		return fmt.Errorf("could not parse request: %w", err)
	}

	logger = logger.
		WithFields(logrus.Fields{
			"request id": req.RequestId,
			"spot":       req.Spot,
		})
	logger.Info("Received request")

	if err := handleRequest(msg, req, coms, logger); err != nil {
		logger = logger.WithError(err)

		sendError := common.SendError(string(msg.Data()), err, req.RequestId, GROUP, coms)
		if sendError != nil {
			logger = logger.WithField("send error", sendError)
			logger.Errorf("Could not send out error")

			return err
		}

		if err := msg.Ack(); err != nil {
			return err
		}
	}

	return nil
}

func handleRequest(
	msg jetstream.Msg,
	req messages.HorizonRequest,
	coms *common.Communications,
	logger *logrus.Entry,
) error {
	radius := 500
	key := common.HorizonKey(req.Spot.Loc, radius)

	logger = logger.WithField("key", key)

	loc := location.Location{
		Latitude:  req.Spot.Loc.Lat,
		Longitude: req.Spot.Loc.Lon,
	}
	logrus.WithFields(logrus.Fields{
		"request": req.RequestId,
		"spot":    req.Spot,
		"key":     key,
		"log":     loc,
		"radius":  radius,
	}).Infof(
		"Computing horizon",
	)
	hor := horizon.NewHorizon(&loc, radius)

	logger.Trace("Storing horizon in cache")
	if _, err := coms.KvHor.Put(coms.Ctx, key, hor.AltitudeToBytes()); err != nil {
		return fmt.Errorf("could not store horizon in cache: %w", err)
	}

	logger.Trace("Setting in compute to false")
	if err := common.SetHorizonInCompute(key, false, coms); err != nil {
		return fmt.Errorf("could not set in compute to false: %w", err)
	}

	logger.Trace("Sending out horizon key")
	if err := common.ForwardHorizonKey(msg, key, coms); err != nil {
		return fmt.Errorf("could not send out horizon key: %w", err)
	}

	if err := msg.Ack(); err != nil {
		return err
	}

	return nil
}
