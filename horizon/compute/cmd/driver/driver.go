package main

import (
	"encoding/json"

	"github.com/nats-io/nats.go"
	"sunangel/horizon/common"
	"sunangel/horizon/messages"
)

func main() {
	// Connect to a server
	nc, _ := nats.Connect(nats.DefaultURL)

	test_spot_msg := messages.HorizonRequest{
		Part: messages.Part{
			Id: 0,
			Of: 1,
		},
		Spot: messages.Spot{
			Dir:  0.,
			Kind: "bench",
			Loc: messages.Location{
				Lat: 48.818611,
				Lon: 9.587340,
			},
		},
	}

	payload, err := json.Marshal(test_spot_msg)
	if err != nil {
		panic(err)
	}

	// Simple Publisher
	err = nc.Publish(common.REQ_GET_Q, []byte(payload))
	if err != nil {
		panic(err)
	}

	nc.Close()
}
