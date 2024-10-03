package main

import (
	"context"

	"dagger/sunangel/internal/dagger"
)

func (m *Sunangel) LocalManualTesting(
	ctx context.Context,
	source *dagger.Directory,
) (*dagger.Container, error) {
	natsService := dag.Container().
		From("nats").
		WithExposedPort(4222).
		WithExposedPort(8222).
		WithDefaultArgs([]string{
			"--jetstream",
			"--name", "main",
			"--http_port", "8222",
		}).
		AsService()

	_, err := buildSpotFinderService(ctx, source, natsService).Start(ctx)
	if err != nil {
		return nil, err
	}

	_, err = buildHorizonGetService(ctx, source, natsService).Start(ctx)
	if err != nil {
		return nil, err
	}

	_, err = buildHorizonComputeService(ctx, source, natsService).Start(ctx)
	if err != nil {
		return nil, err
	}

	_, err = buildSkyServiceService(ctx, source, natsService).Start(ctx)
	if err != nil {
		return nil, err
	}

	apiExecutable := buildRustServiceExecutable(ctx, source, "api")

	return createServiceContainer(apiExecutable).
			WithServiceBinding("nats", natsService).
			WithEnvVariable("NATS_HOST", "nats").
			WithEnvVariable("RUST_LOG", "debug").
			WithExposedPort(6660).
			WithEntrypoint([]string{"/server"}),
		nil
}

func buildSpotFinderService(
	ctx context.Context,
	source *dagger.Directory,
	natsService *dagger.Service,
) *dagger.Service {
	return buildRustService(
		buildRustServiceExecutable(ctx, source, "spot-finder"),
		natsService,
	)
}

func buildSkyServiceService(
	ctx context.Context,
	source *dagger.Directory,
	natsService *dagger.Service,
) *dagger.Service {
	return buildRustService(
		buildRustServiceExecutable(ctx, source, "sky-service"),
		natsService,
	)
}

func buildRustService(
	executable *dagger.File,
	natsService *dagger.Service,
) *dagger.Service {
	return createServiceContainer(executable).
		WithServiceBinding("nats", natsService).
		WithEnvVariable("NATS_HOST", "nats").
		WithEnvVariable("RUST_LOG", "debug").
		WithExec([]string{"/server"}).
		AsService()
}

func buildHorizonGetService(
	ctx context.Context,
	source *dagger.Directory,
	natsService *dagger.Service,
) *dagger.Service {
	return buildGoService(
		buildGoServiceExecutable(ctx, source, "horizon/get", "horizon-get"),
		natsService,
	)
}

func buildHorizonComputeService(
	ctx context.Context,
	source *dagger.Directory,
	natsService *dagger.Service,
) *dagger.Service {
	return buildGoService(
		buildGoServiceExecutable(ctx, source, "horizon/compute", "horizon-compute"),
		natsService,
	)
}

func buildGoService(
	executable *dagger.File,
	natsService *dagger.Service,
) *dagger.Service {
	return createServiceContainer(executable).
		WithServiceBinding("nats", natsService).
		WithEnvVariable("NATS_HOST", "nats").
		WithExec([]string{"/server"}).
		AsService()
}
