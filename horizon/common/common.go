package common

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"strconv"
	"sunangel/horizon/messages"

	"github.com/nats-io/nats.go/jetstream"
	uuid "github.com/satori/go.uuid"
	"github.com/sirupsen/logrus"
)

const (
	HOR_STORE_NAME  = "horizons"
	COMP_STORE_NAME = "horizons-in-computation"

	SPOT_STREAM = "SPOTS"
	REQ_GET_Q   = SPOT_STREAM + ".get-horizon"

	REQ_COMP_Q = SPOT_STREAM + ".compute-horizon"

	RES_OUT_STREAM = "HORIZONS"
	RES_OUT_Q      = RES_OUT_STREAM + ".sunsets"

	ERR_STREAM = "ERRORS"
)

type Communications struct {
	Ctx    context.Context
	Js     jetstream.JetStream
	KvHor  jetstream.KeyValue
	KvComp jetstream.KeyValue
}

func (c *Communications) Close() {
	c.Js.Conn().Close()
}

func ForwardHorizonKey(
	msg jetstream.Msg,
	key string,
	coms *Communications,
) error {
	var msgData map[string]any
	if err := json.Unmarshal(msg.Data(), &msgData); err != nil {
		return err
	}

	msgData["horizon"] = key

	msgPayload, err := json.Marshal(msgData)
	if err != nil {
		return err
	}

	if _, err := coms.Js.Publish(coms.Ctx, RES_OUT_Q, msgPayload); err != nil {
		return err
	}

	return nil
}

func HandleError(
	input string,
	err error,
	requestId string,
	sender string,
	coms *Communications,
) {
	sendError := SendError(input, err, requestId, sender, coms)
	if sendError != nil {
		logrus.
			WithField("request", input). // TODO: double check content of inut
			WithError(sendError).
			Errorf("Could not send out error '%s'", err)
	}
}

// ???

func SendError(
	input string,
	err error,
	requestId string,
	sender string,
	coms *Communications,
) error {
	errorMsg := messages.Error{
		Input:     input,
		Reason:    err.Error(),
		RequestId: requestId,
		Sender:    sender,
	}

	payload, err := json.Marshal(errorMsg)
	if err != nil {
		return err
	}

	_, err = coms.Js.Publish(
		coms.Ctx,
		fmt.Sprintf("%s.%s", ERR_STREAM, requestId),
		payload,
	)
	return err
}

// Horizon key and in compute

func HorizonKey(loc messages.Location, radius int) string {
	id := uuid.NewV5(uuid.UUID{}, fmt.Sprintf(
		// One deg ~ 111 000 m
		"lat: %.5f, lon: %5f, rad: %d",
		loc.Lat, loc.Lon, radius,
	))
	return fmt.Sprint("horizon-v1.0.0-", id)
}

func IsHorizonInCompute(
	key string,
	coms *Communications,
) (bool, error) {
	logrus.Tracef("Checking whether horizon %s is in compute", key)
	isInComputeEntry, err := coms.KvComp.Get(coms.Ctx, key)
	if err != nil {
		logrus.WithError(err).Trace("Received errror when checking for horizon")
		if !IsKeyDoesntExistsError(err) {
			logrus.Tracef("Error %s was not a KeyDoesntExistError", err)
			return false, err
		}

		logrus.Trace("Received error was KeyDoesntExistError, returning false")
		return false, nil
	}

	return DecodeIsIncomputeEntry(isInComputeEntry)
}

func DecodeIsIncomputeEntry(
	entry jetstream.KeyValueEntry,
) (bool, error) {
	logrus.Trace("Decoding entry is in compute")
	if entry.Operation() == jetstream.KeyValueDelete {
		logrus.Trace("Operation was KeyValueDelete")
		return false, nil
	}

	logrus.Tracef("Parsing bool from %s", string(entry.Value()))
	isInCompute, err := strconv.ParseBool(
		string(entry.Value()),
	)
	return isInCompute, err
}

func IsKeyDoesntExistsError(err error) bool {
	return errors.Is(err, jetstream.ErrKeyNotFound) ||
		errors.Is(err, jetstream.ErrKeyDeleted)
}

func SetHorizonInCompute(
	key string,
	val bool,
	coms *Communications,
) error {
	var err error
	if val {
		_, err = coms.KvComp.Put(coms.Ctx, key, []byte(strconv.FormatBool(val)))
	} else {
		err = coms.KvComp.Delete(coms.Ctx, key)
	}
	return err
}
