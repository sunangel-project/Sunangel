package messages

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
