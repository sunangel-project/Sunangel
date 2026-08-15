package com

import (
	"fmt"

	uuid "github.com/satori/go.uuid"
	"codeberg.org/energiesandsuch/horizon/location"
)

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

func HorizonKey(loc location.Location, radius int) string {
	id := uuid.NewV5(uuid.UUID{}, fmt.Sprintf(
		// One deg ~ 111 000 m
		"lat: %.5f, lon: %5f, rad: %d",
		loc.Latitude, loc.Longitude, radius,
	))
	return fmt.Sprint("horizon-v1.0.0-", id)
}
