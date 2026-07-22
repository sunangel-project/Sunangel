package main

import (
	"context"

	"dagger/sunangel/internal/dagger"
)

const (
	RustVersion     = "1.96"
	GoVersion       = "1.26"
	GolangCiVersion = "2.12"
	BunVersion      = "1.3"
	GitPagesVersion = "1.10.0"

	AlpineVersion = "3.24"
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
	return goBuilder().Vet(ctx, source)
}

// Checks that the rust code compiles
func (m *Sunangel) CheckRust(
	ctx context.Context,
	// +defaultPath="/"
	source *dagger.Directory,
) error {
	return rustBuilder().Check(ctx, source)
}

// Run linters
func (m *Sunangel) Lint(
	ctx context.Context,
	// +defaultPath="/"
	source *dagger.Directory,
) error {
	if err := goBuilder().Lint(ctx, source); err != nil {
		return err
	}

	if err := rustBuilder().Lint(ctx, source); err != nil {
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
	if err := goBuilder().Test(ctx, source); err != nil {
		return err
	}

	if err := rustBuilder().Test(ctx, source); err != nil {
		return err
	}

	return nil
}

// Build frontend
func (m *Sunangel) BuildFrontend(
	ctx context.Context,
	// +defaultPath="/front"
	source *dagger.Directory,
) *dagger.Directory {
	return bunBuilder(source).
		WithExec([]string{"bun", "install"}).
		WithExec([]string{"bun", "run", "build"}).
		Directory("dist")
}

// Deploy frontend
func (m *Sunangel) DeployFrontend(
	ctx context.Context,
	// +defaultPath="/front"
	source *dagger.Directory,
	token *dagger.Secret,
) error {
	dist := m.BuildFrontend(ctx, source)

	site := "https://sunn.cloudsftp.de/"
	server := "pages.energiesandsuch.com"

	return dag.GitPages(dagger.GitPagesOpts{
		GitPagesVersion: GitPagesVersion,
	}).Deploy(ctx, dist, token, site, server)
}
