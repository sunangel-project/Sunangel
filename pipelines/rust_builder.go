package main

import (
	"dagger/sunangel/internal/dagger"
)

func rustBuilder() *dagger.Rust {
	return dag.Rust(dagger.RustOpts{
		RustVersion:   RustVersion,
		AlpineVersion: AlpineVersion,
		Packages:      []string{"openssl-dev", "openssl-libs-static"},
	})
}
