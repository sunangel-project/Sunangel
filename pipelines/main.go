package main

import (
	"context"

	"dagger/sunangel/internal/dagger"
)

const (
	RustVersion = "1.94"
	GoVersion   = "1.26"
	BunVersion  = "1.3"

	AlpineVersion = "3.23"
)

type Sunangel struct{}

// Build backend and run extensive testing
func (m *Sunangel) BuildAndTestBackend(
	ctx context.Context,
	// +defaultPath="/"
	source *dagger.Directory,
) error {
	r := rustBuilder()

	err := r.Check(ctx, source)
	if err != nil {
		return err
	}

	err = r.Test(ctx, source)
	if err != nil {
		return err
	}

	err = r.Lint(ctx, source)
	if err != nil {
		return err
	}

	return nil
}

// Checks that all code compiles
func (m *Sunangel) Check(
	ctx context.Context,
	// +defaultPath="/"
	source *dagger.Directory,
) error {
	_, err := m.CheckGo(ctx, source)
	if err != nil {
		return err
	}

	err = m.CheckRust(ctx, source)
	return err
}

// Checks that the go code compiles
func (m *Sunangel) CheckGo(
	ctx context.Context,
	// +defaultPath="/"
	source *dagger.Directory,
) (string, error) {
	return cachedGoBuilder(source).
		WithExec([]string{"go", "vet", "./..."}).
		Stdout(ctx)
}

// Checks that the rust code compiles
func (m *Sunangel) CheckRust(
	ctx context.Context,
	// +defaultPath="/"
	source *dagger.Directory,
) error {
	r := rustBuilder()
	return r.Check(ctx, source)
}

// Run linters
func (m *Sunangel) Lint(
	ctx context.Context,
	// +defaultPath="/"
	source *dagger.Directory,
) error {
	if _, err := cachedGoBuilder(source).
		WithExec([]string{"golangci-lint", "run"}).
		Stdout(ctx); err != nil {
		return err
	}

	r := rustBuilder()
	if err := r.Lint(ctx, source); err != nil {
		return err
	}

	return nil
}

// Run unit tests
func (m *Sunangel) Test(
	ctx context.Context,
	// +defaultPath="/"
	source *dagger.Directory,
) error {
	_, err := cachedGoBuilder(source).
		WithExec([]string{"go", "test", "./..."}).
		Stdout(ctx)

	if err != nil {
		return err
	}

	r := rustBuilder()
	if err := r.Test(ctx, source); err != nil {
		return err
	}

	return nil
}
