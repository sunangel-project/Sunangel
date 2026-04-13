package main

import "dagger/sunangel/internal/dagger"

const (
	user = "appuser"
)

func createServiceContainer(
	executable *dagger.File,
) *dagger.Container {
	return dag.Container().
		From("alpine:"+AlpineVersion).
		WithExec([]string{"adduser", user, "-D"}).
		WithUser(user).
		WithFile("/server", executable)
}
