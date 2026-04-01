package main

import (
	"context"
	"fmt"

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

// TODO: also test before merging
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
		if dir == nil { // TODO: how to handle, error?
			continue
		}

		dirs = append(dirs, dir)
	}

	rustDir, err := mergeDirectories(ctx, dirs)
	if err != nil {
		return nil, fmt.Errorf("could not merge directories: %w", err)
	}

	return rustDir, nil
}

func mergeDirectories(
	ctx context.Context,
	dirs []*dagger.Directory,
) (*dagger.Directory, error) {
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

func mergeDirectories2(
	ctx context.Context,
	first *dagger.Directory,
	second *dagger.Directory,
) (*dagger.Directory, error) {
	entries, err := second.Entries(ctx)
	if err != nil {
		return nil, fmt.Errorf("could not get entries from second directory: %w", err)
	}

	for _, path := range entries {
		first, err = copyPath(ctx, first, second, path)
		if err != nil {
			return nil, fmt.Errorf("could not copy at path '%s': %w", path, err)
		}
	}

	return first, nil
}

func copyPath(
	ctx context.Context,
	first *dagger.Directory,
	second *dagger.Directory,
	path string,
) (*dagger.Directory, error) {
	fileType, err := second.Stat(path).FileType(ctx)
	if err != nil {
		return nil, fmt.Errorf("could not get file type of path: %w", err)
	}

	switch fileType {
	case dagger.FileTypeDirectory:
		directory := second.Directory(path)
		first = first.WithDirectory(path, directory)

	case dagger.FileTypeRegular:
		file := second.File(path)
		first = first.WithFile(path, file)

	case dagger.FileTypeSymlink:
		return nil, fmt.Errorf("symlink at")

	case dagger.FileTypeUnknown:
		return nil, fmt.Errorf("unknown file type")

	default:
		return nil, fmt.Errorf("unexpected file type: %#v", fileType)
	}

	return first, nil
}
