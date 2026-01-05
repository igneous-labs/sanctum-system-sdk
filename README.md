# sanctum-system-sdk

SDK for the [solana system program](https://docs.rs/solana-system-program/latest/solana_system_program/).

Currently only implements a subset of what we need for our programs.

## Structure

- `sanctum-system-core` a no-std, minimal dependencies crate defining common base types and procedures portable to different environments (onchain and offchain). All the other crates below build on top of it.
- `sanctum-system-jiminy` CPI bindings for use onchain with [jiminy](https://github.com/igneous-labs/jiminy)

## Development

This section contains dev info for people who wish to work on the library.

### Solana Versions

#### Toolchain

```sh
$ cargo-build-sbf --version
solana-cargo-build-sbf 3.1.5
platform-tools v1.52
rustc 1.89.0
```
