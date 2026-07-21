package main

import (
	"context"
	"fmt"

	"dagger/sunangel/internal/dagger"
)

func (m *Sunangel) DeployBackend(
	ctx context.Context,
	host, user string,
	key *dagger.Secret,
) error {
	c := dag.Container().
		From("alpine:" + AlpineVersion).
		WithExec([]string{"apk", "add", "--no-cache", "openssh-client-default"})

	mountedKeyPath := "/tmp/mounted-key"
	keyPath := "/tmp/key"

	_, err := c.
		WithMountedSecret(mountedKeyPath, key).
		WithExec([]string{
			"sh", "-c",
			`tr -d "\r" < ` + mountedKeyPath + ` > ` + keyPath,
		}).
		WithExec([]string{
			"sh", "-c",
			`echo -e "\n" >> ` + keyPath,
		}).
		WithExec([]string{
			"sh", "-c",
			`chmod 600 ` + keyPath,
		}).
		WithExec([]string{
			"sh", "-c",
			fmt.Sprintf(
				`ssh -o StrictHostKeyChecking=accept-new -i %s %s@%s ./deploy.sh`,
				keyPath, user, host,
			),
		}).
		Sync(ctx)

	return err
}
