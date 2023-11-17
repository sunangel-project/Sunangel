package main

import (
	"sunangel/horizon/messages"
	"sunangel/messaging"
)

const (
	HOR_STORE_NAME  = "horizons"
	COMP_STORE_NAME = "horizons-in-computation"
)

func main() {
	nc := messaging.Connect()
	defer nc.Close()
	js := messaging.JetStream(nc)

	kvHor := messaging.ConnectOrCreateKV(js, HOR_STORE_NAME)
	kvComp := messaging.ConnectOrCreateKV(js, COMP_STORE_NAME)

	requests := make(chan messages.HorizonRequest)
	compRequests := make(chan messages.HorizonRequest)
	horizons := make(chan messages.HorizonResult)
	go handleRequests(requests, compRequests, horizons)

}

func handleRequests(
	reuests <-chan messages.HorizonRequest,
	comp chan<- messages.HorizonRequest,
	results chan<- messages.HorizonResult,
) {

}
