package main

import (
	"context"
	"fmt"

	"dagger/sunangel/internal/dagger"
)

const (
	golangciVersion = "v1.64"
	golangciLintURL = "github.com/golangci/golangci-lint/cmd/golangci-lint@" + golangciVersion
)

func buildGoServiceImage(
	ctx context.Context,
	source *dagger.Directory,
	path string,
	name string,
) *dagger.Container {
	executable := buildGoServiceExecutable(ctx, source, path, name)

	return createServiceContainer(executable).
		WithEntrypoint([]string{"/server"})
}

func buildGoServiceExecutable(
	ctx context.Context,
	source *dagger.Directory,
	path string,
	name string,
) *dagger.File {
	return cachedGoBuilder(source).
		WithExec([]string{
			"go", "build", "-o", name,
			fmt.Sprintf("%s/%s.go", path, name),
		}).
		File(name)
}

func cachedGoBuilder(
	source *dagger.Directory,
) *dagger.Container {
	return dag.Container().
		From(fmt.Sprintf("golang:%s-alpine%s", GoVersion, AlpineVersion)).

		// Caches
		WithMountedCache("/go/pkg/mod", dag.CacheVolume("go-mod")).
		WithEnvVariable("GOMODCACHE", "/go/pkg/mod").
		WithMountedCache("/go/build-cache", dag.CacheVolume("go-build")).
		WithEnvVariable("GOCACHE", "/go/build-cache").

		// Linter
		WithExec([]string{"go", "install", golangciLintURL}).

		// Execute tests
		WithDirectory("/src", source).
		WithWorkdir("/src")
}
