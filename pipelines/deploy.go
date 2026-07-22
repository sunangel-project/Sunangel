package main

import (
	"context"

	"dagger/sunangel/internal/dagger"
)

func (m *Sunangel) DeployBackend(
	ctx context.Context,
	key *dagger.Secret,
	host, user string,
) error {
	return dag.
		SSH(key, host, user).
		Run(ctx, "./deploy.sh")
}
