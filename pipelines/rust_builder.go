package main

import (
	"context"
	"fmt"

	"dagger/sunangel/internal/dagger"
)

func buildRustServiceImage(
	ctx context.Context,
	source *dagger.Directory,
	name string,
) *dagger.Container {
	executable := buildRustServiceExecutable(ctx, source, name)

	return createServiceContainer(executable).
		WithEntrypoint([]string{"/server"})
}

func buildRustServiceExecutable(
	ctx context.Context,
	source *dagger.Directory,
	name string,
) *dagger.File {
	outputFile := "/" + name

	return cachedRustBuilder(source).
		WithExec([]string{"cargo", "build", "--release", "-p", name}).
		WithExec([]string{"cp", "target/release/" + name, outputFile}).
		File(outputFile)
}

func cachedRustBuilder(
	source *dagger.Directory,
) *dagger.Container {
	source = source.WithoutDirectory("target")

	return dag.Container().
		From(fmt.Sprintf("rust:%s-alpine%s", RustVersion, AlpineVersion)).
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
