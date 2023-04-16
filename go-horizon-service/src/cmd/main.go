package main

import (
	"encoding/json"
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
	// ec := messaging.EncodedConnection(nc)
	// defer ec.Close()
	js := messaging.JetStream(nc)
	kv := messaging.KeyValueHorizon(js)

	wg := sync.WaitGroup{}
	wg.Add(1)

	sub, err := nc.QueueSubscribe(messaging.IN_Q, "go-horizon", func(msg *nats.Msg) {
		err := handle_message(msg, kv, nc)
		if err != nil {
			// TODO: print or sum
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
	nc *nats.Conn,
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

	horizon_key, err := handle_spot_message(&spot_msg, kv)
	if err != nil {
		return err
	}

	unstructured_msg["horizon"] = horizon_key

	out_payload, err := json.Marshal(unstructured_msg)
	if err != nil {
		return err
	}
	nc.Publish(messaging.OUT_Q, out_payload)

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
	if err != nil { // TODO: Better error handling, first check if key does not exist
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
