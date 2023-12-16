# Commands

## **gdpack `add`**

Add the dependency at the provided `URI`; can be a filepath or a URL to a git repository.

### Usage

`gdpack add [OPTIONS] <URI>`

### Options

- `-d`, `--dev` — add a development-only dependency (will not be propagated to dependents' installs)
- `-n`, `--name` — install the addon named `NAME` from a multi-addon dependency; if omitted, assumed to be repository name or filepath base name
- `-p`, `--project <PATH>` — a `PATH` to the Godot project containing the manifest
- `-t`, `--target <TARGET>` — add the dependency only for `TARGET` (can be specified more than once)

#### git-specific options

- `--branch <BRANCH>` — use a git `BRANCH` version (only used with a git repository `URI`)
- `--commit <COMMIT>` — use a git `COMMIT` version (only used with a git repository `URI`)
- `--tag <TAG>` — use a git `TAG` version (only used with a git repository `URI`)

### Arguments

- `<URI>` — a filepath or URL to the addon (can be git repository)
  - Example values:
    - `../third_party/godot-next`
    - `../third_party/godot-next/addons/godot-next`
    - `https://github.com/godot-extended-libraries/godot-next`

## **gdpack `init`**

Create a new `gdpack.toml` manifest for the _Godot_ project.

### Usage

`gdpack init [OPTIONS]`

### Options

- `-p`, `--project <PATH>` — a `PATH` to the Godot project containing the manifest

## **gdpack `install`**

Install addon dependencies into the _Godot_ project's `addons/` directory.

### Usage

`gdpack install [OPTIONS]`

### Options

- `-p`, `--project <PATH>` — a `PATH` to the Godot project containing the manifest
- `--prod`, `--production` — don't install development dependencies
- `-t`, `--target <TARGET>` — install dependencies only for `TARGET` (can be specified more than once)

## **gdpack `remove`**

Remove the specified dependency.

### Usage

`gdpack remove [OPTIONS] <NAME>`

### Options

- `-p`, `--project <PATH>` — a `PATH` to the Godot project containing the manifest
- `-t`, `--target <TARGET>` — remove the dependency only for `TARGET` (can be specified more than once)

### Arguments

- `<NAME>` — the name of an installed addon
  - Example values:
    - `godot-next`

## **gdpack `replace`**

Replace a dependency with one at the provided `URI`; can be a filepath or a URL to a git repository.

### Usage

`gdpack replace [OPTIONS] <NAME> <URI>`

### Options

- `-d`, `--dev` — replace a development-only dependency (will not be propagated to dependents' installs)
- `-n`, `--name` — replace with the addon named `NAME` from a multi-addon dependency; if omitted, assumed to be repository name or filepath base name
- `-p`, `--project <PATH>` — a `PATH` to the Godot project containing the manifest
- `-t`, `--target <TARGET>` — replace the dependency only for `TARGET` (can be specified more than once)

#### git-specific options

- `--branch <BRANCH>` — use a git `BRANCH` version (only used with a git repository `URI`)
- `--commit <COMMIT>` — use a git `COMMIT` version (only used with a git repository `URI`)
- `--tag <TAG>` — use a git `TAG` version (only used with a git repository `URI`)

### Arguments

- `<NAME>` — the name of an installed addon to replace
  - Example values:
    - `godot-next`
- `<URI>` — a filepath or URL to the addon (can be git repository)
  - Example values:
    - `../third_party/godot-next`
    - `../third_party/godot-next/addons/godot-next`
    - `https://github.com/godot-extended-libraries/godot-next`
