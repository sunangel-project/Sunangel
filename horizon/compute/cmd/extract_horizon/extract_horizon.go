package main

import (
	"flag"
	"fmt"
	"os"
)

func main() {
	var lat, lon float64
	var radius int

	flag.Float64Var(&lat, "lat", 0., "Latitude of Coordinates")
	flag.Float64Var(&lon, "lon", 0., "Longitude of Coordinates")
	flag.IntVar(&radius, "r", 500, "Radius of Horizon")

	flag.Parse()

	fmt.Print("Currently out of order")
	os.Exit(1)

	/*
		nc := messaging.Connect()
		defer nc.Close()
		js := messaging.JetStream(nc)
		kv := messaging.KeyValueHorizon(js)

		key := storage.HorizonKey(*location.NewLocation(lat, lon), radius)
		horEntry, err := kv.Get(key)
		if err != nil {
			panic(err)
		}

		alt, err := horizon.AltitudeFromBytes(horEntry.Value())
		if err != nil {
			panic(err)
		}

		b, _ := json.Marshal(alt)
		fmt.Print(string(b))
	*/
}
