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
)

const (
	HOR_STORE_NAME  = "horizons"
	COMP_STORE_NAME = "horizons-in-computation"

	SPOT_STREAM = "SPOTS"
	REQ_GET_Q   = SPOT_STREAM + ".get-horizon"

	REQ_COMP_Q = SPOT_STREAM + ".compute-horizon"

	RES_OUT_STREAM = "HORIZONS"
	RES_OUT_Q      = RES_OUT_STREAM + ".sunsets"

	ERR_STREAM = "ERROR"
)

type Communications struct {
	Ctx    context.Context
	Js     jetstream.JetStream
	KvHor  jetstream.KeyValue
	KvComp jetstream.KeyValue
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
		fmt.Sprintf("%s.%s", ERR_STREAM, sender),
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
	isInComputeEntry, err := coms.KvComp.Get(coms.Ctx, key)
	if err != nil {
		if IsKeyDoesntExistsError(err) {
			return false, nil
		}

		return false, err
	}

	return DecodeIsIncomputeEntry(isInComputeEntry)
}

func DecodeIsIncomputeEntry(
	entry jetstream.KeyValueEntry,
) (bool, error) {
	if entry.Operation() == jetstream.KeyValueDelete {
		return false, nil
	}

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
