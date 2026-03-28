package main

import (
	"context"

	"dagger/sunangel/internal/dagger"
)

const (
	RustVersion = "1.94"
	GoVersion   = "1.26"

	AlpineVersion = "3.23"
)

type Sunangel struct{}

// Build backend and run extensive testing
func (m *Sunangel) BuildAndTestBackend(
	ctx context.Context,
	// +defaultPath="/"
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
	// +defaultPath="/"
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
) (string, error) {
	return cachedRustBuilder(source).
		WithExec([]string{"cargo", "check"}).
		Stdout(ctx)
}

// Run linters
func (m *Sunangel) Lint(
	ctx context.Context,
	// +defaultPath="/"
	source *dagger.Directory,
) (string, error) {
	goLintResults, err := cachedGoBuilder(source).
		WithExec([]string{"golangci-lint", "run"}).
		Stdout(ctx)

	if err != nil {
		return "", err
	}

	rustLintResults, err := cachedRustBuilder(source).
		WithExec([]string{"cargo", "clippy", "--", "-D", "warnings"}).
		Stdout(ctx)

	if err != nil {
		return "", err
	}

	return goLintResults + "\n" + rustLintResults, nil
}

// Run unit tests
func (m *Sunangel) Test(
	ctx context.Context,
	// +defaultPath="/"
	source *dagger.Directory,
) (string, error) {
	goTestResults, err := cachedGoBuilder(source).
		WithExec([]string{"go", "test", "./..."}).
		Stdout(ctx)

	if err != nil {
		return "", err
	}

	rustTestResults, err := cachedRustBuilder(source).
		WithExec([]string{"cargo", "test"}).
		Stdout(ctx)

	if err != nil {
		return "", err
	}

	return goTestResults + "\n" + rustTestResults, nil
}
