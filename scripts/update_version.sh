#!/bin/bash

api_versions_file="api/src/api.rs"
frontend_versions_file="front/src/main.ts"
readme="README.md"

update_api_version="false"

display_help() {
    echo "usage: $0 [--api] <version>"
    exit 2
}

if [ $# -gt 1 ]; then
    case "$1" in
        "--api")
            update_api_version="true"
            shift
            ;;
        *)
            display_help
            ;;
    esac
fi

version="$1"

#TODO: validate version?

function update_version_in_api_file_fun {
    local prefix="$1"
    sed -i "s#\(${prefix}_VERSION: &str = \)\".*\"#\1\"$version\"#" $api_versions_file
}

function join_by {
  local d=${1-} f=${2-}
  if shift 2; then
    printf %s "$f" "${@/#/$d}"
  fi
}

function update_version_in_readme_fun {
    local name="$1"
    local version="$(join_by "--" ${version//-/ })"
    sed -i "s#\($name-\).*\(-blue\)#\1$version\2#" $readme
}

function update_version_in_frontend_version_file_fun {
    sed -i "s#\(version = \)\".*\"#\1\"$version\"#" $frontend_versions_file
}

function update_version_fun {
    update_version_in_api_file_fun "BACKEND"
    update_version_in_readme_fun "Version"
    update_version_in_frontend_version_file_fun
}

function update_api_version_fun {
    update_version_in_api_file_fun "API"
    update_version_in_readme_fun "API"
}

if [ "$update_api_version" = "true" ]; then
    update_api_version_fun
else
    update_version_fun
fi

