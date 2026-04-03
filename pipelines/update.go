package main

import (
	"context"

	"dagger/sunangel/internal/dagger"
)

func updateRust(
	ctx context.Context,
	source *dagger.Directory,
) *dagger.Directory {
	return cachedRustBuilder(source).
		WithExec([]string{"cargo", "update"}).
		Directory("").
		Filter(dagger.DirectoryFilterOpts{
			Include: []string{"Cargo.lock"},
		})
}

func updateGo(
	ctx context.Context,
	source *dagger.Directory,
) *dagger.Directory {
	return cachedGoBuilder(source).
		WithExec([]string{"go", "get", "-u", "./..."}).
		Directory("").
		Filter(dagger.DirectoryFilterOpts{
			Include: []string{
				"*.sum",
				"*.mod",
			},
		})
}

func updateTypescript(
	ctx context.Context,
	source *dagger.Directory,
) *dagger.Directory {
	return cachedBunBuilder(source).
		WithWorkdir("front").
		WithExec([]string{"bun", "update"}).
		WithWorkdir("..").
		Directory("").
		Filter(dagger.DirectoryFilterOpts{
			Include: []string{"front/bun.lock"},
		})
}

type directoryGenerator = func(context.Context, *dagger.Directory) *dagger.Directory

// Update all dependencies
func (m *Sunangel) UpdateDependencies(
	ctx context.Context,
	// +defaultPath="/"
	source *dagger.Directory,
) (*dagger.Directory, error) {
	var dirs []*dagger.Directory
	for _, f := range []directoryGenerator{
		updateRust,
		updateTypescript,
		updateGo,
	} {
		dir := f(ctx, source)
		if dir == nil {
			continue
		}

		dirs = append(dirs, dir)
	}

	merged := dag.MergeDirs().Merge(dirs)
	return merged, nil
}
