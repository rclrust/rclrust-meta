#!/bin/bash

set -euo pipefail

function main() {
    cd "$(dirname "$0")"

    local repo_root
    repo_root=$(pwd)

    for distro in foxy galactic humble rolling; do
        mkdir -p generated
        : >generated/${distro}.rs

        for repo in rcl rcutils rmw; do
            git -C thirdparty/${repo} switch ${distro}
        done
        (
            cd rcl-bindgen
            BINDGEN_OUT=${repo_root}/generated/${distro}.rs THIRDPARTY=${repo_root}/thirdparty cargo run
        )
    done
}

main "$@"
