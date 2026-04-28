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
	// +defaultPath="/"
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
		{m.ImageApi(source), "api"},
		{m.ImageHorizonGet(ctx, source), "horizon-get"},
		{m.ImageHorizonCompute(ctx, source), "horizon-compute"},
		{m.ImageSkyService(source), "sky-service"},
		{m.ImageSpotFinder(source), "spot-finder"},
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
	// +defaultPath="/"
	source *dagger.Directory,
) *dagger.Container {
	return rustBuilder().
		BuildImage(source, "api").
		WithExposedPort(6660)
}

// Build image of the spot-finder service
func (m *Sunangel) ImageSpotFinder(
	// +defaultPath="/"
	source *dagger.Directory,
) *dagger.Container {
	return rustBuilder().BuildImage(source, "spot-finder")
}

// Build image of the sky-service service
func (m *Sunangel) ImageSkyService(
	// +defaultPath="/"
	source *dagger.Directory,
) *dagger.Container {
	return rustBuilder().BuildImage(source, "sky-service")
}

// Build image of the horizon-get service
func (m *Sunangel) ImageHorizonGet(
	ctx context.Context,
	// +defaultPath="/"
	source *dagger.Directory,
) *dagger.Container {
	return goBuilder().
		BuildImage(source, "horizon-compute", dagger.GoBuildImageOpts{
			Path: "horizon/compute",
		})
}

// Build image of the horizon-compute service
func (m *Sunangel) ImageHorizonCompute(
	ctx context.Context,
	// +defaultPath="/"
	source *dagger.Directory,
) *dagger.Container {
	return goBuilder().
		BuildImage(source, "horizon-get", dagger.GoBuildImageOpts{
			Path: "horizon/get",
		})
}
