package main

import (
	"encoding/json"

	"github.com/nats-io/nats.go"
	"github.com/sunangel-project/go-horizon-service/src/messaging"
)

func main() {
	// Connect to a server
	nc, _ := nats.Connect(nats.DefaultURL)

	test_spot_msg := messaging.SpotMessage{
		Part: messaging.PartSubMessage{
			Id: 0, Of: 1,
		},
		Spot: messaging.SpotSubMessage{
			Dir:  0.,
			Kind: "bench",
			Loc: messaging.Location{
				Lat: 48.818600,
				Lon: 9.587340,
			},
		},
	}

	payload, err := json.Marshal(test_spot_msg)
	if err != nil {
		panic(err)
	}

	// Simple Publisher
	nc.Publish(messaging.IN_Q, []byte(payload))

	nc.Close()
}
