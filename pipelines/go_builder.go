package main

import (
	"dagger/sunangel/internal/dagger"
)

func goBuilder() *dagger.Go {
	return dag.Go(dagger.GoOpts{
		GoVersion:       GoVersion,
		GolangCiVersion: GolangCiVersion,
		AlpineVersion:   AlpineVersion,
	})
}

func goAlpine() *dagger.Alpine {
	return dag.Alpine(dagger.AlpineOpts{
		AlpineVersion: AlpineVersion,
	})
}
