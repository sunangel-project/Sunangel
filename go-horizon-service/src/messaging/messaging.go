package messaging

import (
	"errors"
	"log"
	"time"

	"github.com/nats-io/nats.go"
)

const STORE_NAME = "horizons"

const IN_Q = "SPOTS.horizon"
const GROUP = "horizon-service"

const OUT_STREAM = "HORIZONS"
const OUT_SUB_SUNSETS = OUT_STREAM + ".sunset"

const ERR_STREAM = "ERRORS"
const ERR_SUB = ERR_STREAM + "." + GROUP

type PartSubMessage struct {
	Id uint `json:"id"`
	Of uint `json:"of"`
}

type Location struct {
	Lat float64 `json:"lat"`
	Lon float64 `json:"lon"`
}

type SpotSubMessage struct {
	Dir  float64  `json:"dir"`
	Kind string   `json:"kind"`
	Loc  Location `json:"loc"`
}

type SpotMessage struct {
	Part      PartSubMessage `json:"part"`
	Spot      SpotSubMessage `json:"spot"`
	RequestId string         `json:"request_id"`
}

type OutMessage struct {
	Part      PartSubMessage `json:"part"`
	Spot      SpotSubMessage `json:"spot"`
	RequestId string         `json:"request_id"`
	Horizon   string         `json:"horizon"`
}

func Connect() *nats.Conn {
	// TODO: get host from env
	nc, err := nats.Connect(nats.DefaultURL)
	if err != nil {
		panic(err)
	}

	return nc
}

func EncodedConnection(nc *nats.Conn) *nats.EncodedConn {
	ec, err := nats.NewEncodedConn(nc, nats.JSON_ENCODER)
	if err != nil {
		panic(err)
	}

	return ec
}

func JetStream(nc *nats.Conn) nats.JetStreamContext {
	js, err := nc.JetStream()
	if err != nil {
		panic(err)
	}

	return js
}

func KeyValueHorizon(js nats.JetStreamContext) nats.KeyValue {
	kv, err := js.KeyValue(STORE_NAME)
	if err != nil {
		log.Printf("Bucket %s not found, creating", STORE_NAME)
		kv, err = js.CreateKeyValue(&nats.KeyValueConfig{
			Bucket: STORE_NAME,
		})

		if err != nil {
			panic(err)
		}

	}

	return kv
}

func Setup_streams(js nats.JetStreamContext) error {
	output_stream_config := &nats.StreamConfig{
		Name:     OUT_STREAM,
		Subjects: []string{OUT_SUB_SUNSETS},
		MaxAge:   time.Hour,
	}
	_, err := js.StreamInfo(OUT_STREAM)
	if err != nil {
		if !errors.Is(err, nats.ErrStreamNotFound) {
			return err
		}

		js.AddStream(output_stream_config)
	}

	error_stream_config := &nats.StreamConfig{
		Name:     ERR_STREAM,
		Subjects: []string{ERR_SUB},
		MaxAge:   time.Hour,
	}
	_, err = js.StreamInfo(ERR_STREAM)
	if err != nil {
		if !errors.Is(err, nats.ErrStreamNotFound) {
			return err
		}

		js.AddStream(error_stream_config)
	}

	return nil
}
