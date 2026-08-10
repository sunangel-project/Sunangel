package main

import (
	"context"
	"fmt"

	"dagger/sunangel/internal/dagger"
)

// Publish all images
func (m *Sunangel) PublishImages(
	ctx context.Context,
	// +defaultPath="/"
	source *dagger.Directory,
	tag string,
	actor string,
	token *dagger.Secret,
) error {
	publishImage := func(image *dagger.Container, name string) error {
		url := fmt.Sprintf("codeberg.org/energiesandsuch/%s:%s", name, tag)

		_, err := image.
			WithRegistryAuth("codeberg.org", actor, token).
			Publish(ctx, url)
		if err != nil {
			return err
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
	return buildRustImage(source, "api").
		WithExposedPort(6660)
}

// Build image of the spot-finder service
func (m *Sunangel) ImageSpotFinder(
	// +defaultPath="/"
	source *dagger.Directory,
) *dagger.Container {
	return buildRustImage(source, "spot-finder")
}

// Build image of the sky-service service
func (m *Sunangel) ImageSkyService(
	// +defaultPath="/"
	source *dagger.Directory,
) *dagger.Container {
	return buildRustImage(source, "sky-service")
}

// Build image of the horizon-get service
func (m *Sunangel) ImageHorizonGet(
	ctx context.Context,
	// +defaultPath="/"
	source *dagger.Directory,
) *dagger.Container {
	return buildGoImage(source, "horizon/compute", "horizon-compute")
}

// Build image of the horizon-compute service
func (m *Sunangel) ImageHorizonCompute(
	ctx context.Context,
	// +defaultPath="/"
	source *dagger.Directory,
) *dagger.Container {
	return buildGoImage(source, "horizon/get", "horizon-get")
}

func buildRustImage(
	source *dagger.Directory,
	pkg string,
) *dagger.Container {
	binary := rustBuilder().
		BuildExecutable(
			source, pkg,
			dagger.RustBuildExecutableOpts{
				Target: RustBinaryTargetx86_64Alpine,
			},
		)

	return rustAlpine().
		ServiceContainer(binary, pkg, dagger.AlpineServiceContainerOpts{
			Platform: TargetContainerPlatform,
		})
}

func buildGoImage(
	source *dagger.Directory,
	path string,
	name string,
) *dagger.Container {
	binary := goBuilder().
		Compile(source, name, dagger.GoCompileOpts{
			Path: path,
			Os:   GoBinaryTargetOS,
			Arch: GoBinaryTargetArch,
		})

	return goAlpine().
		ServiceContainer(binary, name, dagger.AlpineServiceContainerOpts{
			Platform: TargetContainerPlatform,
		})
}
