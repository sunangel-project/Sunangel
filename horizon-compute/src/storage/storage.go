package storage

import (
	"fmt"

	uuid "github.com/satori/go.uuid"
	"github.com/sunangel-project/horizon/location"
)

func HorizonKey(loc location.Location, radius int) string {
	id := uuid.NewV5(uuid.UUID{}, fmt.Sprintf(
		// One deg ~ 111 000 m
		"lat: %.5f, lon: %5f, rad: %d",
		loc.Latitude, loc.Longitude, radius,
	))
	return fmt.Sprint("horizon-v1.0.0-", id)
}
