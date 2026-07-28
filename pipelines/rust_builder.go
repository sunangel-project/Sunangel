package main

import (
	"dagger/sunangel/internal/dagger"
)

func rustBuilder() *dagger.Rust {
	return dag.Rust(dagger.RustOpts{
		RustVersion:   RustVersion,
		AlpineVersion: AlpineVersion,
		Packages:      RustAlpinePackages,
	})
}

func rustAlpine() *dagger.Alpine {
	return dag.Alpine(dagger.AlpineOpts{
		AlpineVersion: AlpineVersion,
		Packages:      RustAlpinePackages,
	})
}
