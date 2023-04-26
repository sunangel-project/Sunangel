package main

import (
	"encoding/json"
	"errors"
	"fmt"
	"log"
	"sync"

	"github.com/nats-io/nats.go"
	uuid "github.com/satori/go.uuid"
	"github.com/sunangel-project/go-horizon-service/src/messaging"
	"github.com/sunangel-project/horizon"
	"github.com/sunangel-project/horizon/location"
)

func main() {
	nc := messaging.Connect()
	defer nc.Close()
	js := messaging.JetStream(nc)
	kv := messaging.KeyValueHorizon(js)

	log.Println("Setting up all streams")
	err := messaging.Setup_streams(js)
	if err != nil {
		panic(err)
	}

	wg := sync.WaitGroup{}
	wg.Add(1)

	log.Printf("Subscribing to queue %v\n", messaging.IN_Q)
	sub, err := js.QueueSubscribe(messaging.IN_Q, messaging.GROUP, func(msg *nats.Msg) {
		log.Println("Received message")
		err := handle_message(msg, kv, js)
		if err != nil {
			log.Printf("Error %v occured when reading message %v\n", err, msg)
		}
	})
	if err != nil {
		panic(err)
	}

	// Wait for messages to come in
	wg.Wait()

	sub.Unsubscribe()

	// Drain connection (Preferred for responders)
	nc.Drain()
}

func handle_message(
	msg *nats.Msg,
	kv nats.KeyValue,
	js nats.JetStreamContext,
) error {
	var unstructured_msg map[string]any

	err := json.Unmarshal(msg.Data, &unstructured_msg)
	if err != nil {
		return err
	}

	var spot_msg messaging.SpotMessage
	err = json.Unmarshal(msg.Data, &spot_msg)
	if err != nil {
		return err
	}

	log.Printf("Decoded message: %+v", spot_msg)

	horizon_key, err := handle_spot_message(&spot_msg, kv)
	if err != nil {
		return err
	}

	unstructured_msg["horizon"] = horizon_key

	out_payload, err := json.Marshal(unstructured_msg)
	if err != nil {
		return err
	}
	js.Publish(messaging.OUT_SUB_SUNSETS, out_payload)

	return nil
}

func handle_spot_message(
	spot_msg *messaging.SpotMessage,
	kv nats.KeyValue,
) (string, error) {
	loc := location.Location{
		Latitude:  spot_msg.Spot.Loc.Lat,
		Longitude: spot_msg.Spot.Loc.Lon,
	}
	radius := 500

	// One deg ~ 111 000 m
	id := uuid.NewV5(uuid.UUID{}, fmt.Sprintf(
		"lat: %.5f, lon: %5f, rad: %d",
		loc.Latitude, loc.Longitude, radius,
	))
	key := fmt.Sprint("horizon-v1.0.0-", id)

	_, err := kv.Get(key)
	if err != nil {
		if !errors.Is(err, nats.ErrKeyNotFound) {
			return "", err
		}

		log.Print("Didn't find horizon")
		hor := horizon.NewHorizon(&loc, radius)

		kv.Create(key, hor.AltitudeToBytes())
	} else {
		log.Print("Found horizon")

		/* Not needed ?
		altitude, err := horizon.AltitudeFromBytes(hor_entry.Value())
		if err != nil {
			panic(err)
		}

		hor = horizon.NewHorizonWithAltitude(
			&loc,
			radius,
			altitude,
		)
		*/
	}

	return key, nil
}
