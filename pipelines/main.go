package main

import (
	"context"

	"dagger/sunangel/internal/dagger"
)

const (
	RustVersion = "1.81"
	GoVersion   = "1.23"

	AlpineVersion = "3.20"
)

type Sunangel struct{}

// Run the pipeline
func (m *Sunangel) Pipeline(
	ctx context.Context,
	source *dagger.Directory,
) error {
	err := m.Check(ctx, source)
	if err != nil {
		return err
	}

	_, err = m.Test(ctx, source)
	if err != nil {
		return err
	}

	_, err = m.Lint(ctx, source)
	if err != nil {
		return err
	}

	return nil
}

// Checks that all code compiles
func (m *Sunangel) Check(
	ctx context.Context,
	source *dagger.Directory,
) error {
	_, err := m.CheckGo(ctx, source)
	if err != nil {
		return err
	}

	_, err = m.CheckRust(ctx, source)
	return err
}

// Checks that the go code compiles
func (m *Sunangel) CheckGo(
	ctx context.Context,
	source *dagger.Directory,
) (string, error) {
	return "", nil

	/* TODO: fix go code
	return cachedGoBuilder(source).
		WithExec([]string{"go", "vet", "./..."}).
		Stdout(ctx)
	*/
}

// Checks that the rust code compiles
func (m *Sunangel) CheckRust(
	ctx context.Context,
	source *dagger.Directory,
) (string, error) {
	return cachedRustBuilder(source).
		WithExec([]string{"cargo", "check"}).
		Stdout(ctx)
}

// Run linters
func (m *Sunangel) Lint(
	ctx context.Context,
	source *dagger.Directory,
) (string, error) {
	// TODO: lint go source code

	return cachedRustBuilder(source).
		WithExec([]string{"cargo", "clippy", "--", "-D", "warnings"}).
		Stdout(ctx)
}

// Run unit tests
func (m *Sunangel) Test(
	ctx context.Context,
	source *dagger.Directory,
) (string, error) {
	// TODO: run go tests

	return cachedRustBuilder(source).
		WithExec([]string{"cargo", "test"}).
		Stdout(ctx)
}

// Build the image of the api service
func (m *Sunangel) ImageApi(
	ctx context.Context,
	source *dagger.Directory,
) *dagger.Container {
	executable := buildRustServiceExecutable(ctx, source, "api")

	return createServiceContainer(executable).
		WithExposedPort(6660).
		WithExec([]string{"/server"})
}

// Build the image of the spot-finder service
func (m *Sunangel) ImageSpotFinder(
	ctx context.Context,
	source *dagger.Directory,
) *dagger.Container {
	return buildRustServiceImage(ctx, source, "spot-finder")
}

// Build the image of the sky-service service
func (m *Sunangel) ImageSkyService(
	ctx context.Context,
	source *dagger.Directory,
) *dagger.Container {
	return buildRustServiceImage(ctx, source, "sky-service")
}

// Build image of the horizon-get service
func (m *Sunangel) ImageHorizonGet(
	ctx context.Context,
	source *dagger.Directory,
) *dagger.Container {
	return buildGoServiceImage(ctx, source, "horizon/get", "horizon-get")
}

// Build image of the horizon-compute service
func (m *Sunangel) ImageHorizonCompute(
	ctx context.Context,
	source *dagger.Directory,
) *dagger.Container {
	return buildGoServiceImage(ctx, source, "horizon/compute", "horizon-compute")
}
