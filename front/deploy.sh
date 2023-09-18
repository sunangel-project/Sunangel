#!/usr/bin/env sh

set -e

npm run build

cd dist

echo 'sunn.cloudsftp.de' > CNAME

git init
git add -A
git commit -m 'deploy'

git push -f git@github.com:sunangel-project/Sunangel.git latest:gh-pages

cd -
