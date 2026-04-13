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

	apiExecutable := buildRustServiceExecutable(ctx, source, "api")

	return createServiceContainer(apiExecutable).
			WithServiceBinding("nats", natsService).
			WithFile("/.env", envFile).
			WithExposedPort(6660).
			WithEntrypoint([]string{"/server"}),
		nil
}

func buildSpotFinderService(
	ctx context.Context,
	source *dagger.Directory,
	natsService *dagger.Service,
	envFile *dagger.File,
) *dagger.Service {
	return buildRustService(
		buildRustServiceExecutable(ctx, source, "spot-finder"),
		natsService,
		envFile,
	)
}

func buildSkyServiceService(
	ctx context.Context,
	source *dagger.Directory,
	natsService *dagger.Service,
	envFile *dagger.File,
) *dagger.Service {
	return buildRustService(
		buildRustServiceExecutable(ctx, source, "sky-service"),
		natsService,
		envFile,
	)
}

func buildRustService(
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
