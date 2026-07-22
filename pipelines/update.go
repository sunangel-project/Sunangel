package main

import (
	"context"
	"fmt"

	"dagger/sunangel/internal/dagger"
)

// Run renovate
func (m *Sunangel) RunRenovate(
	ctx context.Context,
	forgeToken *dagger.Secret,
	githubToken *dagger.Secret,
) (string, error) {
	platform := "forgejo"
	endpoint := "https://codeberg.org/api/v1/"
	repository := "sunangel-project/sunangel"

	config := fmt.Sprintf(`
		module.exports = {
		  "repositories": [
		    "%s"
		  ]
		}
		`,
		repository,
	)

	return dag.Container().
		From("renovate/renovate:"+RenovateVersion).
		WithNewFile("config.js", config).
		WithSecretVariable("RENOVATE_TOKEN", forgeToken).
		WithSecretVariable("GITHUB_COM_TOKEN", githubToken).
		WithExec([]string{
			"renovate",
			"--platform", platform,
			"--endpoint", endpoint,
		}).
		Stdout(ctx)

}
