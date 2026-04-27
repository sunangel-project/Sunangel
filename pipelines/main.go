package main

import (
	"context"

	"dagger/sunangel/internal/dagger"
)

const (
	RustVersion     = "1.94"
	GoVersion       = "1.26"
	GolangCiVersion = "2.11"
	BunVersion      = "1.3"

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
	err := m.CheckGo(ctx, source)
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
) error {
	g := dag.Go()
	return g.Vet(ctx, source)
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
	g := dag.Go()
	if err := g.Lint(ctx, source); err != nil {
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
	g := goBuilder()
	if err := g.Test(ctx, source); err != nil {
		return err
	}

	r := rustBuilder()
	if err := r.Test(ctx, source); err != nil {
		return err
	}

	return nil
}
