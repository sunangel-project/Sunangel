package main

import (
	"encoding/json"
	"errors"
	"log"
	"sync"

	"github.com/nats-io/nats.go"
	"github.com/sunangel-project/go-horizon-service/src/messaging"
	"github.com/sunangel-project/go-horizon-service/src/storage"
	"github.com/sunangel-project/horizon"
	"github.com/sunangel-project/horizon/location"
)

func main() {
	nc := messaging.Connect()
	defer nc.Close()
	js := messaging.JetStream(nc)
	kv := messaging.KeyValueHorizon(js)

	log.Println("Setting up all streams")
	err := messaging.SetupStreams(js)
	if err != nil {
		panic(err)
	}

	wg := sync.WaitGroup{}
	wg.Add(1)

	log.Printf("Subscribing to queue %v\n", messaging.IN_Q)
	sub, err := js.QueueSubscribe(messaging.IN_Q, messaging.GROUP, func(msg *nats.Msg) {
		log.Println("Received message")
		err := handleMessage(msg, kv, js)
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

func handleMessage(
	msg *nats.Msg,
	kv nats.KeyValue,
	js nats.JetStreamContext,
) error {
	var unstructuredMsg map[string]any

	err := json.Unmarshal(msg.Data, &unstructuredMsg)
	if err != nil {
		return err
	}

	var spot_msgspotMsg messaging.SpotMessage
	err = json.Unmarshal(msg.Data, &spot_msgspotMsg)
	if err != nil {
		return err
	}

	log.Printf("Decoded message: %+v", spot_msgspotMsg)

	horKey, err := handleSpotMessage(&spot_msgspotMsg, kv)
	if err != nil {
		return err
	}

	unstructuredMsg["horizon"] = horKey

	outPayload, err := json.Marshal(unstructuredMsg)
	if err != nil {
		return err
	}
	js.Publish(messaging.OUT_SUB_SUNSETS, outPayload)

	return nil
}

func handleSpotMessage(
	spot_msg *messaging.SpotMessage,
	kv nats.KeyValue,
) (string, error) {
	loc := location.Location{
		Latitude:  spot_msg.Spot.Loc.Lat,
		Longitude: spot_msg.Spot.Loc.Lon,
	}
	radius := 500
	key := storage.HorizonKey(loc, radius)
	log.Printf("Horizon Key: %s", key)

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
	}

	return key, nil
}
