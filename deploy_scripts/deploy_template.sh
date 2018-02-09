#!/bin/bash

# run only from travis.
# automated build script

# parameters to set up in travis:
# $GH_TOKEN -> github token
# $REPO_LINK -> repository link. format: github.com/:username/:reponame
# $GENERATED_FOLDER_NAME -> folder with generated html
# $AUTHOR -> author of repository
# $EMAIL -> email

echo "> check if should deploy"

if [[ "$TRAVIS_BRANCH" != "master" || "$TRAVIS_RUST_VERSION" != "nightly" || "$TRAVIS_OS_NAME" != "linux" ]]; then
    echo "This commit was made against '$TRAVIS_BRANCH' with '$TRAVIS_RUST_VERSION' on '$TRAVIS_OS_NAME'."
    echo "Instead of 'master' branch with 'nightly' on 'linux'!"
    echo "Not deploying!"
    exit 0
fi

echo "> generating website"
cargo run

echo "> deploying"
REV=$(git rev-parse --short HEAD)
cd $GENERATED_FOLDER_NAME
git init
git remote add upstream "https://$GH_TOKEN@$REPO_LINK"
git config user.name "Travis ($AUTHOR)"
git config user.email "$EMAIL"
git add -A .
git commit -qm "Build page at ${TRAVIS_REPO_SLUG}@${REV}"

echo "> pushing"
git push -q upstream HEAD:refs/heads/pages --force