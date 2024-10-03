package main

import "dagger/sunangel/internal/dagger"

func createServiceContainer(
	executable *dagger.File,
) *dagger.Container {
	return dag.Container().
		From("alpine:"+AlpineVersion).
		WithFile("/server", executable)
}
