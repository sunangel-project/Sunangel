package main

import (
	"context"
	"fmt"
	"strings"

	"dagger/sunangel/internal/dagger"
)

// Publish all images
func (m *Sunangel) PublishImages(
	ctx context.Context,
	source *dagger.Directory,
	version string,
	actor string,
	token *dagger.Secret,
) error {
	version = strings.TrimLeft(version, "v")

	tags := []string{"latest", version}

	publishImage := func(image *dagger.Container, name string) error {
		for _, tag := range tags {
			url := fmt.Sprintf("ghcr.io/sunangel-project/%s:%s", name, tag)

			_, err := image.
				WithRegistryAuth("ghcr.io", actor, token).
				Publish(ctx, url)
			if err != nil {
				return err
			}
		}

		return nil
	}

	for _, pair := range []struct {
		image *dagger.Container
		name  string
	}{
		{m.ImageApi(ctx, source), "api"},
		{m.ImageHorizonGet(ctx, source), "horizon-get"},
		{m.ImageHorizonCompute(ctx, source), "horizon-compute"},
		{m.ImageSkyService(ctx, source), "sky-service"},
	} {
		err := publishImage(pair.image, pair.name)
		if err != nil {
			return err
		}
	}

	return nil
}

// Build image of the api service
func (m *Sunangel) ImageApi(
	ctx context.Context,
	source *dagger.Directory,
) *dagger.Container {
	executable := buildRustServiceExecutable(ctx, source, "api")

	return createServiceContainer(executable).
		WithExposedPort(6660).
		WithEntrypoint([]string{"/server"})
}

// Build image of the spot-finder service
func (m *Sunangel) ImageSpotFinder(
	ctx context.Context,
	source *dagger.Directory,
) *dagger.Container {
	return buildRustServiceImage(ctx, source, "spot-finder")
}

// Build image of the sky-service service
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
