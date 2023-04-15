package messaging

import (
	"log"

	"github.com/nats-io/nats.go"
)

const STORE_NAME = "horizons"

const IN_Q = "spots"
const GROUP = "horizon-service"

const OUT_Q = "horizons"
const ERR_Q = "error"

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
	Part PartSubMessage `json:"part"`
	Spot SpotSubMessage `json:"spot"`
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
