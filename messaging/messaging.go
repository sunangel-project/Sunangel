package messaging

import (
	"errors"
	"fmt"
	"log"
	"os"

	"github.com/nats-io/nats.go"
)

func Connect() *nats.Conn {
	natsURL := os.Getenv("NATS_HOST")
	if natsURL == "" {
		natsURL = nats.DefaultURL
	}

	nc, err := nats.Connect(natsURL)
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

func CreateStream(js nats.JetStreamContext, name string) error {
	stream_config := &nats.StreamConfig{
		Name:     name,
		Subjects: []string{name, fmt.Sprintf("%s.*", name)},
	}
	_, err := js.StreamInfo(name)
	if err != nil {
		if !errors.Is(err, nats.ErrStreamNotFound) {
			return err
		}

		js.AddStream(stream_config)
	}

	return nil
}

func ConnectOrCreateKV(js nats.JetStreamContext, name string) nats.KeyValue {
	kv, err := js.KeyValue(name)
	if err != nil {
		log.Printf("Bucket %s not found, creating", name)
		kv, err = js.CreateKeyValue(&nats.KeyValueConfig{
			Bucket: name,
		})

		if err != nil {
			panic(err)
		}

	}

	return kv
}
