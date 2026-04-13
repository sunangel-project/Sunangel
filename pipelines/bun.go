package main

import (
	"fmt"

	"dagger/sunangel/internal/dagger"
)

func cachedBunBuilder(
	source *dagger.Directory,
) *dagger.Container {
	source = source.WithoutDirectory("target")

	return dag.Container().
		From(fmt.Sprintf("oven/bun:%s-alpine", BunVersion)).
		WithExec([]string{"apk", "update"}).
		WithDirectory("/src", source).
		WithWorkdir("/src")

	// Caches
	//WithMountedCache("/cache/cargo", dag.CacheVolume("bun-packages")).
	//WithEnvVariable("CARGO_HOME", "/cache/cargo").
	//WithMountedCache("target", dag.CacheVolume("bun-target"))
}
