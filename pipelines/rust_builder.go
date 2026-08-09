package main

import (
	"dagger/sunangel/internal/dagger"
)

func rustBuilder() *dagger.Rust {
	return dag.Rust(dagger.RustOpts{
		RustVersion:   RustVersion,
		AlpineVersion: AlpineVersion,
		Targets:       []string{RustBinaryTargetx86_64Alpine},
		Packages:      RustAlpinePackages,
	})
}

func rustAlpine() *dagger.Alpine {
	return dag.Alpine(dagger.AlpineOpts{
		AlpineVersion: AlpineVersion,
		Packages:      RustAlpinePackages,
	})
}
