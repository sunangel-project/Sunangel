package main

import (
	"context"

	"dagger/sunangel/internal/dagger"
)

type Sunangel struct{}

// Builds a service, returns executable
func (m *Sunangel) BuildService(
	ctx context.Context,
	source *dagger.Directory,
	name string,
) *dagger.File {
	outputFile := "/" + name

	return cachedRustBuilder(source).
		// Build
		WithExec([]string{"cargo", "build", "--release", "-p", name}).
		WithExec([]string{"cp", "target/release/" + name, outputFile}).
		File(outputFile)
}

func (m *Sunangel) Lint(
	ctx context.Context,
	source *dagger.Directory,
) (string, error) {
	return cachedRustBuilder(source).
		WithExec([]string{"cargo", "clippy", "--", "-D", "warnings"}).
		Stdout(ctx)
}

func cachedRustBuilder(
	source *dagger.Directory,
) *dagger.Container {
	source = source.WithoutDirectory("target")

	return dag.Container().
		From("rust:1.81-alpine").
		WithExec([]string{"apk", "update"}).
		WithExec([]string{
			"apk", "add", "--no-cache",
			"pkgconfig", "musl-dev",
			"openssl-dev", "openssl-libs-static",
		}).
		WithExec([]string{"rustup", "component", "add", "clippy"}).
		WithDirectory("/src", source).
		WithWorkdir("/src").

		// Caches
		WithMountedCache("/cache/cargo", dag.CacheVolume("rust-packages")).
		WithEnvVariable("CARGO_HOME", "/cache/cargo").
		WithMountedCache("target", dag.CacheVolume("rust-target"))
}
