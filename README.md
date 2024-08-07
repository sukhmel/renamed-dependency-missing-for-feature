# renamed-dependency-missing-for-feature
Possible bug in Rust's cargo, when a dependency is renamed AND gated behind feature AND published to private registry

When crate is referenced via `path`, dependencies are correctly found, see `usage-path`

When crate is published to private registry and then referenced via registry, renamed optional dependencies are missing, see `usage-registry`
