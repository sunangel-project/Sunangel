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

	_, err := buildSpotFinderService(source, natsService, envFile).Start(ctx)
	if err != nil {
		return nil, err
	}

	_, err = buildHorizonGetService(source, natsService, envFile).Start(ctx)
	if err != nil {
		return nil, err
	}

	_, err = buildHorizonComputeService(source, natsService, envFile).Start(ctx)
	if err != nil {
		return nil, err
	}

	_, err = buildSkyServiceService(source, natsService, envFile).Start(ctx)
	if err != nil {
		return nil, err
	}

	return buildRustImageIntegration(
		source,
		"api",
		natsService,
		envFile,
	).WithExposedPort(6660), nil
}

func buildSpotFinderService(
	source *dagger.Directory,
	natsService *dagger.Service,
	envFile *dagger.File,
) *dagger.Service {
	return buildRustImageIntegration(
		source,
		"spot-finder",
		natsService,
		envFile,
	).AsService()
}

func buildSkyServiceService(
	source *dagger.Directory,
	natsService *dagger.Service,
	envFile *dagger.File,
) *dagger.Service {
	return buildRustImageIntegration(
		source,
		"sky-service",
		natsService,
		envFile,
	).AsService()
}

func buildRustImageIntegration(
	source *dagger.Directory,
	pkg string,
	natsService *dagger.Service,
	envFile *dagger.File,
) *dagger.Container {
	binary := rustBuilder().
		BuildExecutable(source, pkg)

	return dag.Alpine(dagger.AlpineOpts{
		AlpineVersion: AlpineVersion,
	}).
		ServiceContainer(binary, pkg).
		WithServiceBinding("nats", natsService).
		WithFile("/.env", envFile)
}

func buildHorizonGetService(
	source *dagger.Directory,
	natsService *dagger.Service,
	envFile *dagger.File,
) *dagger.Service {
	return buildGoImageIntegration(
		source,
		"horizon-get",
		"horizon/get",
		natsService,
		envFile,
	).AsService()
}

func buildHorizonComputeService(
	source *dagger.Directory,
	natsService *dagger.Service,
	envFile *dagger.File,
) *dagger.Service {
	return buildGoImageIntegration(
		source,
		"horizon-compute",
		"horizon/compute",
		natsService,
		envFile,
	).AsService()
}

func buildGoImageIntegration(
	source *dagger.Directory,
	name string,
	path string,
	natsService *dagger.Service,
	envFile *dagger.File,
) *dagger.Container {
	binary := goBuilder().
		Compile(source, name, dagger.GoCompileOpts{
			Path: path,
		})

	return goAlpine().
		ServiceContainer(binary, name).
		WithServiceBinding("nats", natsService).
		WithFile("/.env", envFile)
}
