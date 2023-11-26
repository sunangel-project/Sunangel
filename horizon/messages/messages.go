package messages

import (
	"errors"
	"fmt"

	"github.com/nats-io/nats.go"
	uuid "github.com/satori/go.uuid"
)

type Part struct {
	Id uint `json:"id"`
	Of uint `json:"of"`
}

type Location struct {
	Lat float64 `json:"lat"`
	Lon float64 `json:"lon"`
}

type Spot struct {
	Dir  float64  `json:"dir"`
	Kind string   `json:"kind"`
	Loc  Location `json:"loc"`
}

type HorizonRequest struct {
	Part      Part   `json:"part"`
	Spot      Spot   `json:"spot"`
	RequestId string `json:"request_id"`
}

type HorizonResult struct {
	Part      Part   `json:"part"`
	Spot      Spot   `json:"spot"`
	RequestId string `json:"request_id"`
	Horizon   string `json:"horizon"`
}

func HorizonKey(loc Location, radius int) string {
	id := uuid.NewV5(uuid.UUID{}, fmt.Sprintf(
		// One deg ~ 111 000 m
		"lat: %.5f, lon: %5f, rad: %d",
		loc.Lat, loc.Lon, radius,
	))
	return fmt.Sprint("horizon-v1.0.0-", id)
}

func IsKeyDoesntExistsError(err error) bool {
	return errors.Is(err, nats.ErrKeyNotFound) ||
		errors.Is(err, nats.ErrKeyDeleted)
}
