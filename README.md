# renamed-dependency-missing-for-feature
Possible bug in Rust's cargo, when a dependency is renamed AND gated behind feature AND published to private registry

When crate is referenced via `path`, dependencies are correctly found, see `usage-path`

When crate is published to private registry and then referenced via registry, renamed optional dependencies are missing, see `usage-registry`

# Steps to reproduce:

1. run `kellnr`:
    ```
    docker run --rm -it \
        -p 8000:8000 \
        -e "KELLNR_ORIGIN__HOSTNAME=localhost" ghcr.io/kellnr/kellnr:5.0.0
    ```
2. publish example-dependency
    ```
    cargo publish --registry kellnr -p example-dependency
    ```
3. build packages
    ```
    cargo build
    ```
