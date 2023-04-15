package main

import (
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
	ec := messaging.EncodedConnection(nc)
	defer ec.Close()
	js := messaging.JetStream(nc)
	kv := messaging.KeyValueHorizon(js)

	// Use a WaitGroup to wait for 10 messages to arrive
	wg := sync.WaitGroup{}
	wg.Add(10)

	sub, err := ec.Subscribe(messaging.IN_Q, func(spot_msg *messaging.SpotMessage) {
		handle_message(spot_msg, kv, ec)
		wg.Done()
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
	spot_msg *messaging.SpotMessage,
	kv nats.KeyValue,
	ec *nats.EncodedConn,
) {
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

	out_msg := messaging.OutMessage{
		Part:      spot_msg.Part,
		Spot:      spot_msg.Spot,
		RequestId: spot_msg.RequestId,
		Horizon:   key,
	}

	ec.Publish(messaging.OUT_Q, out_msg)
}
