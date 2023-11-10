# **gdpack** ![GitHub release (with filter)](https://img.shields.io/github/v/release/coffeebeats/gdpack) ![GitHub](https://img.shields.io/github/license/coffeebeats/gdpack) [![Build Status](https://img.shields.io/github/actions/workflow/status/coffeebeats/gdpack/check-commit.yml?branch=main)](https://github.com/coffeebeats/gdpack/actions?query=branch%3Amain+workflow%3Acheck) [![codecov](https://codecov.io/gh/coffeebeats/gdpack/graph/badge.svg)](https://codecov.io/gh/coffeebeats/gdpack)

A small, single-purpose CLI application for managing Godot addons.

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

1. [Follow the instructions](https://go.dev/doc/install) to install Go (see [go.mod](./go.mod) for the minimum required version).
2. Clone the [coffeebeats/gdpack](https://github.com/coffeebeats/gdpack) repository.
3. Install the tools [used below](#code-submission) by following each of their specific installation instructions.

### Code submission

When submitting code for review, ensure the following requirements are met:

> ❕ **NOTE:** These instructions do not persist the tools to your development environment. When regular use is required, follow each tool's individual instructions to install permanent versions.

1. The project is correctly formatted using [goimports](https://pkg.go.dev/golang.org/x/tools/cmd/goimports):

    ```sh
    go run golang.org/x/tools/cmd/goimports@latest -w .
    ```

2. All [golangci-lint](https://golangci-lint.run/) linter warnings are addressed:

    ```sh
    go run github.com/golangci/golangci-lint/cmd/golangci-lint@latest run ./...
    ```

3. All unit tests pass and no data races are found:

    ```sh
    go test -race ./...
    ```

4. The `gdpack` binary successfully compiles with [goreleaser](https://goreleaser.com/) (release artifacts will be available at `./dist`):

    ```sh
    go run github.com/goreleaser/goreleaser@latest release --clean --skip=publish --snapshot
    ```

## **Contributing**

All contributions are welcome! Feel free to file [bugs](https://github.com/coffeebeats/gdpack/issues/new?assignees=&labels=bug&projects=&template=bug-report.md&title=) and [feature requests](https://github.com/coffeebeats/gdpack/issues/new?assignees=&labels=enhancement&projects=&template=feature-request.md&title=) and/or open pull requests.

## **Version history**

See [CHANGELOG.md](https://github.com/coffeebeats/gdpack/blob/main/CHANGELOG.md).

## **License**

[MIT License](https://github.com/coffeebeats/gdpack/blob/main/LICENSE)
