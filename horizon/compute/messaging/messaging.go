package messaging

import (
	"github.com/nats-io/nats.go"
	"sunangel/messaging"
)

const STORE_NAME = "horizons"
const IN_COMPUTATION_STORE_NAME = "computing-horizons"

const IN_Q = "SPOTS.get-horizon"
const GROUP = "horizon-service"

const OUT_STREAM = "SPOTS"
const OUT_SUB_SUNSETS = OUT_STREAM + ".compute-horizon"

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
