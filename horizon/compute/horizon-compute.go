package main

import (
	"encoding/json"
	"errors"
	"log"
	"sync"

	"github.com/nats-io/nats.go"
	uuid "github.com/satori/go.uuid"
	"github.com/sunangel-project/horizon"
	"github.com/sunangel-project/horizon/location"

	"fmt"
	"sunangel/horizon/messages"
	"sunangel/messaging"
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

	var spot_msgspotMsg messages.SpotMessage
	err = json.Unmarshal(msg.Data, &spot_msgspotMsg)
	if err != nil {
		return err
	}

	log.Printf("Decoded message: %+v", spot_msgspotMsg)

	horKey, err := handleSpotMessage(&spot_msgspotMsg, kv)
	// TODO: set horizonincompute
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

// needed later
func setHorizonInCompute(
	key string,
	val bool,
	coms *Communications,
) {
	coms.kvComp.Put(key, []byte(strconv.FormatBool(val)))
}

func handleSpotMessage(
	spotMsg *messages.Spot,
	kv nats.KeyValue,
) (string, error) {
	radius := 500
	key := messages.HorizonKey(spotMsg.Loc, radius)
	log.Printf("Horizon Key: %s", key)

	_, err := kv.Get(key)
	if err != nil {
		if !errors.Is(err, nats.ErrKeyNotFound) {
			return "", err
		}

		log.Print("Didn't find horizon")
		loc := location.Location{
			Latitude:  spotMsg.Loc.Lat,
			Longitude: spotMsg.Loc.Lon,
		}
		hor := horizon.NewHorizon(&loc, radius)

		kv.Create(key, hor.AltitudeToBytes())
	} else {
		log.Print("Found horizon")
	}

	return key, nil
}

const STORE_NAME = "horizons"
const IN_COMPUTATION_STORE_NAME = "horizons-in-computation"

const IN_Q = "SPOTS.compute-horizon"
const GROUP = "horizon-service"

const OUT_STREAM = "HORIZONS"
const OUT_SUB_SUNSETS = OUT_STREAM + ".sunsets"

const ERR_STREAM = "ERRORS"
const ERR_SUB = ERR_STREAM + "." + GROUP

func SetupStreams(js nats.JetStreamContext) error {
	if err := messaging.CreateStream(js, OUT_STREAM); err != nil {
		return err
	}

	if err := messaging.CreateStream(js, ERR_STREAM); err != nil {
		return err
	}

	return nil
}
