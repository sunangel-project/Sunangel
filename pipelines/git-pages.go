package main

import (
	"context"
	"fmt"

	"dagger/sunangel/internal/dagger"
)

func (m *Sunangel) PublishGitPage(
	ctx context.Context,
	dist *dagger.Directory,
	token *dagger.Secret,
) error {
	site := "https://sunn.cloudsftp.de/"
	server := "pages.energiesandsuch.com"

	g := dag.Container().
		From("codeberg.org/git-pages/git-pages-cli:" + GitPagesVersion)

	tokenPlain, err := token.Plaintext(ctx)
	if err != nil {
		return fmt.Errorf("could not get token contents: %w", err)
	}

	_, err = g.
		WithDirectory("/dist", dist).
		WithSecretVariable("TOKEN", token).
		WithExec([]string{
			"git-pages-cli",
			site,
			"--token", tokenPlain,
			"--server", server,
			"--upload-dir", "/dist",
		}).
		Sync(ctx)

	return err
}
