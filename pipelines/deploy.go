package main

import (
	"context"

	"dagger/sunangel/internal/dagger"
)

// Deploy frontend
func (m *Sunangel) DeployFrontend(
	ctx context.Context,
	// +defaultPath="/front"
	source *dagger.Directory,
	token *dagger.Secret,
) error {
	dist := m.BuildFrontend(ctx, source)

	site := "https://sunn.cloudsftp.de/"
	server := "pages.energiesandsuch.com"

	return dag.GitPages(dagger.GitPagesOpts{
		GitPagesVersion: GitPagesVersion,
	}).Deploy(ctx, dist, token, site, server)
}

// Deploy backend
func (m *Sunangel) DeployBackend(
	ctx context.Context,
	key *dagger.Secret,
	host, user string,
) error {
	return dag.
		SSH(key, host, user).
		Run(ctx, "./deploy.sh")
}
