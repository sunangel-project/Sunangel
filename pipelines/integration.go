package main

import (
	"context"

	"dagger/sunangel/internal/dagger"
)

// Instantiates all backend services
func (m *Sunangel) LocalManualTesting(
	ctx context.Context,
	// +defaultPath="/"
	source *dagger.Directory,
) (*dagger.Container, error) {
	envFile := source.File(".env")

	natsService := dag.Container().
		From("nats:latest").
		WithEntrypoint([]string{
			"/nats-server",
			"--jetstream",
			"--name", "main",
			"--http_port", "8222",
		}).
		WithExposedPort(4222).
		WithExposedPort(8222).
		AsService()

	_, err := buildSpotFinderService(ctx, source, natsService, envFile).Start(ctx)
	if err != nil {
		return nil, err
	}

	_, err = buildHorizonGetService(ctx, source, natsService, envFile).Start(ctx)
	if err != nil {
		return nil, err
	}

	_, err = buildHorizonComputeService(ctx, source, natsService, envFile).Start(ctx)
	if err != nil {
		return nil, err
	}

	_, err = buildSkyServiceService(ctx, source, natsService, envFile).Start(ctx)
	if err != nil {
		return nil, err
	}

	return buildRustImage(source, "api", natsService, envFile).WithExposedPort(6660), nil
}

func buildSpotFinderService(
	ctx context.Context,
	source *dagger.Directory,
	natsService *dagger.Service,
	envFile *dagger.File,
) *dagger.Service {
	return buildRustImage(
		source,
		"spot-finder",
		natsService,
		envFile,
	).AsService()
}

func buildSkyServiceService(
	ctx context.Context,
	source *dagger.Directory,
	natsService *dagger.Service,
	envFile *dagger.File,
) *dagger.Service {
	return buildRustImage(
		source,
		"sky-service",
		natsService,
		envFile,
	).AsService()
}

func buildRustImage(
	source *dagger.Directory,
	pkg string,
	natsService *dagger.Service,
	envFile *dagger.File,
) *dagger.Container {
	r := rustBuilder()
	return r.BuildImage(source, pkg).
		WithServiceBinding("nats", natsService).
		WithFile("/.env", envFile)
}

func buildHorizonGetService(
	ctx context.Context,
	source *dagger.Directory,
	natsService *dagger.Service,
	envFile *dagger.File,
) *dagger.Service {
	return buildGoService(
		buildGoServiceExecutable(ctx, source, "horizon/get", "horizon-get"),
		natsService,
		envFile,
	)
}

func buildHorizonComputeService(
	ctx context.Context,
	source *dagger.Directory,
	natsService *dagger.Service,
	envFile *dagger.File,
) *dagger.Service {
	return buildGoService(
		buildGoServiceExecutable(ctx, source, "horizon/compute", "horizon-compute"),
		natsService,
		envFile,
	)
}

func buildGoService(
	executable *dagger.File,
	natsService *dagger.Service,
	envFile *dagger.File,
) *dagger.Service {
	return createServiceContainer(executable).
		WithServiceBinding("nats", natsService).
		WithFile("/.env", envFile).
		WithEntrypoint([]string{"/server"}).
		AsService()
}
