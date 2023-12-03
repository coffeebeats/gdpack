# **gdpack** ![GitHub release (with filter)](https://img.shields.io/github/v/release/coffeebeats/gdpack) ![GitHub](https://img.shields.io/github/license/coffeebeats/gdpack) [![Build Status](https://img.shields.io/github/actions/workflow/status/coffeebeats/gdpack/check-commit.yml?branch=main)](https://github.com/coffeebeats/gdpack/actions?query=branch%3Amain+workflow%3Acheck) [![codecov](https://codecov.io/gh/coffeebeats/gdpack/graph/badge.svg)](https://codecov.io/gh/coffeebeats/gdpack)

A small, single-purpose CLI application for managing Godot addons.

> ⚠️ **WARNING:** This project is in a very early stage. API instability, missing features, and bugs are to be expected for now.

## **How it works**

TODO

## **Getting started**

These instructions will help you install `gdpack` and manage addons for your _Godot_ projects.

### **Example usage**

TODO

### **Installation**

See [docs/installation.md](./docs/installation.md#installation) for detailed instructions on how to download `gdpack`.

## **API Reference**

### **Commands**

See [docs/commands.md](./docs/commands.md) for a detailed reference on how to use each command.

## **Development**

### Setup

The following instructions outline how to get the project set up for local development:

1. [Follow the instructions](https://www.rust-lang.org/tools/install) to install Rust (see [Cargo.toml](./Cargo.toml) for the minimum required version).
2. Clone the [coffeebeats/gdpack](https://github.com/coffeebeats/gdpack) repository.
3. Install the tools [used below](#code-submission) by following each of their specific installation instructions.

### Code submission

When submitting code for review, ensure the following requirements are met:

1. The project is correctly formatted using [rustfmt](https://github.com/rust-lang/rustfmt):

    ```sh
    cargo fmt
    ```

2. All [clippy](https://github.com/rust-lang/rust-clippy) linter warnings are addressed:

    ```sh
    cargo clippy \
        --all-features \
        --all-targets \
        --no-deps \
        -- \
            --deny=warnings
    ```

3. All unit tests pass:

    ```sh
    cargo test \
        --all-features \
        --all-targets \
        --frozen \
        --release
    ```

4. The `gdpack` binary successfully compiles using [Cross](https://github.com/cross-rs/cross) (release artifacts will be available in `./target`). Follow the [installation instructions](https://github.com/cross-rs/cross#installation) to ensure `cross` is installed on the development system.

    ```sh
    cross build \
        --manifest-path=Cargo.toml \
        --profile=release \
        --frozen \
        --all-targets
    ```

## **Contributing**

All contributions are welcome! Feel free to file [bugs](https://github.com/coffeebeats/gdpack/issues/new?assignees=&labels=bug&projects=&template=bug-report.md&title=) and [feature requests](https://github.com/coffeebeats/gdpack/issues/new?assignees=&labels=enhancement&projects=&template=feature-request.md&title=) and/or open pull requests.

## **Version history**

See [CHANGELOG.md](https://github.com/coffeebeats/gdpack/blob/main/CHANGELOG.md).

## **License**

[MIT License](https://github.com/coffeebeats/gdpack/blob/main/LICENSE)
