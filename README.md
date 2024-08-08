# Archived

~~Possible~~ bug in Rust's cargo, when a dependency is renamed AND gated behind feature AND published to private registry. This repository was for https://github.com/rust-lang/cargo/issues/14365, that appeared to be a duplicate of https://github.com/rust-lang/cargo/issues/14321, and is fixed in https://github.com/rust-lang/cargo/issues/14365. Releases 1.78, 1.79, and 1.80 of `cargo` were affected, 1.81 should be released shortly with a fix of this issue. 

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
3. update cargo index:
    ```shell
    cargo update
    ```
4. build packages
    ```
    cargo build
    ```
