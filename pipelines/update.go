package main

import (
	"context"
	"fmt"

	"dagger/sunangel/internal/dagger"
)

// Update all dependencies
func (m *Sunangel) UpdateDependencies(
	ctx context.Context,
	// +defaultPath="/"
	source *dagger.Directory,
) (*dagger.Directory, error) {
	rustDir := cachedRustBuilder(source).
		WithExec([]string{"cargo", "update"}).
		Directory("").
		Filter(dagger.DirectoryFilterOpts{
			Include: []string{"Cargo.lock"},
		})

	goDir := cachedGoBuilder(source).
		WithExec([]string{"go", "get", "-u", "./..."}).
		Directory("").
		Filter(dagger.DirectoryFilterOpts{
			Include: []string{
				"*.sum",
				"*.mod",
			},
		})

	rustDir, err := mergeDirectories(ctx, []*dagger.Directory{
		rustDir,
		goDir,
	})
	if err != nil {
		return nil, fmt.Errorf("could not merge directories: %w", err)
	}

	return rustDir, nil
}

func mergeDirectories(ctx context.Context, dirs []*dagger.Directory) (*dagger.Directory, error) {
	var err error

	if len(dirs) < 2 {
		return nil, fmt.Errorf("need at least 2 directories to merge")
	}

	first, rest := dirs[0], dirs[1:]
	for i, next := range rest {
		first, err = mergeDirectories2(ctx, first, next)
		if err != nil {
			return nil, fmt.Errorf("could not merge direcotory %d: %w", i, err)
		}
	}

	return first, nil
}

func mergeDirectories2(ctx context.Context, first *dagger.Directory, second *dagger.Directory) (*dagger.Directory, error) {
	entries, err := second.Entries(ctx)
	if err != nil {
		return nil, fmt.Errorf("could not get entries from second directory: %w", err)
	}

	for _, path := range entries {
		file := second.File(path)
		first = first.WithFile(path, file)
	}

	return first, nil
}
