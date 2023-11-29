package messaging

import (
	"context"
	"errors"
	"fmt"
	"log"
	"os"

	"github.com/nats-io/nats.go"
	"github.com/nats-io/nats.go/jetstream"
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

func JetStream(nc *nats.Conn) jetstream.JetStream {
	js, err := jetstream.New(nc)
	if err != nil {
		panic(err)
	}

	return js
}

func CreateStream(ctx context.Context, js jetstream.JetStream, name string) error {
	streamConfig := jetstream.StreamConfig{
		Name:     name,
		Subjects: []string{name, fmt.Sprintf("%s.*", name)},
	}
	_, err := js.Stream(ctx, name)
	if err != nil {
		if !errors.Is(err, jetstream.ErrStreamNotFound) {
			return err
		}

		js.CreateStream(ctx, streamConfig)
	}

	return nil
}

func SetupStreams(ctx context.Context, js jetstream.JetStream, names []string) error {
	for _, name := range names {
		if err := CreateStream(ctx, js, name); err != nil {
			return fmt.Errorf("could not initialize stream '%s': %s", name, err)
		}
	}

	return nil
}

func ConnectOrCreateConsumer(
	ctx context.Context,
	stream jetstream.Stream,
	name string,
	conf jetstream.ConsumerConfig,
) (jetstream.Consumer, error) {
	cons, err := stream.Consumer(ctx, name)
	if err != nil {
		if err == jetstream.ErrConsumerDoesNotExist {
			return nil, err
		}

		cons, err = stream.CreateConsumer(ctx, conf)
		if err != nil {
			return nil, err
		}
	}

	return cons, nil
}

func ConnectOrCreateKV(ctx context.Context, js jetstream.JetStream, name string) jetstream.KeyValue {
	kv, err := js.KeyValue(ctx, name)
	if err != nil {
		log.Printf("Bucket %s not found, creating", name)
		kv, err = js.CreateKeyValue(ctx, jetstream.KeyValueConfig{
			Bucket: name,
		})

		if err != nil {
			panic(err)
		}

	}

	return kv
}
