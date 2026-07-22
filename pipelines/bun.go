package main

import (
	"dagger/sunangel/internal/dagger"
)

func bunBuilder(source *dagger.Directory) *dagger.Container {
	return dag.Bun(dagger.BunOpts{
		BunVersion: BunVersion,
		Packages:   []string{"nodejs"},
	}).
		Builder(source)
}
