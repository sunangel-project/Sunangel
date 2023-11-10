package main

import (
	"encoding/json"
	"errors"
	"log"
	"sync"

	"sunangel/horizon/compute/storage"
	"sunangel/messaging"

	"github.com/nats-io/nats.go"
	"github.com/sunangel-project/horizon"
	"github.com/sunangel-project/horizon/location"
)

func main() {
	nc := messaging.Connect()
	defer nc.Close()
	js := messaging.JetStream(nc)
	kv := messaging.ConnectOrCreateKV(js, STORE_NAME)

	log.Println("Setting up all streams")
	err := SetupStreams(js)
	if err != nil {
		panic(err)
	}

	wg := sync.WaitGroup{}
	wg.Add(1)

	log.Printf("Subscribing to queue %v\n", IN_Q)
	sub, err := js.QueueSubscribe(IN_Q, GROUP, func(msg *nats.Msg) {
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
	js.Publish(OUT_SUB_SUNSETS, outPayload)

	return nil
}

func handleSpotMessage(
	spot_msg *SpotMessage,
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

const STORE_NAME = "horizons"
const IN_COMPUTATION_STORE_NAME = "computing-horizons"

const IN_Q = "SPOTS.compute-horizon"
const GROUP = "horizon-service"

const OUT_STREAM = "HORIZONS"
const OUT_SUB_SUNSETS = OUT_STREAM + ".sunsets"

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

func SetupStreams(js nats.JetStreamContext) error {
	if err := messaging.CreateStream(js, OUT_STREAM); err != nil {
		return err
	}

	if err := messaging.CreateStream(js, ERR_STREAM); err != nil {
		return err
	}

	return nil
}
