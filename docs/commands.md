# Commands

## **gdpack `add`**

Add the dependency at the provided `URI`; can be a filepath or a URL to a git repository.

### Usage

`gdpack add [OPTIONS] <URI>`

### Options

- `-d`, `--dev` — add a development-only dependency (will not be propagated to dependents' installs)
- `-n`, `--name` — install the addon named `NAME` from a multi-addon dependency; if omitted, assumed to be repository name or filepath base name
- `-p`, `--path <PATH>` — a `PATH` to the Godot project containing the manifest
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

- `-p`, `--path <PATH>` — a `PATH` to the Godot project containing the manifest

## **gdpack `install`**

Install addon dependencies into the _Godot_ project's `addons/` directory.

### Usage

`gdpack install [OPTIONS]`

### Options

- `-p`, `--path <PATH>` — a `PATH` to the Godot project containing the manifest
- `--prod`, `--production` — don't install development dependencies
- `-t`, `--target <TARGET>` — install dependencies only for `TARGET` (can be specified more than once)

## **gdpack `patch`**

Prepares a dependency for patching.

### Usage

`gdpack patch [OPTIONS] <NAME>`

### Options

- `-d`, `--dir <DIR>`, `--directory <DIR>` — a directory `DIR` to edit the dependency in
- `-p`, `--path <PATH>` — a `PATH` to the Godot project containing the manifest
- `-t`, `--target <TARGET>` — remove the dependency only for `TARGET` (can be specified more than once)

### Arguments

- `<NAME>` — the name of an installed addon
  - Example values:
    - `godot-next`

## **gdpack `patch-commit`**

Applies the edits made via `gdpack patch`.

### Usage

`gdpack patch-commit [OPTIONS] <DIR>`

### Arguments

- `<DIR>` — the directory `DIR` in which the dependency was edited
  - Example values:
    - `/tmp/gdpack-patch-godot-next-XXXXXXXXX`

## **gdpack `remove`**

Remove the specified dependency.

### Usage

`gdpack remove [OPTIONS] <NAME>`

### Options

- `-p`, `--path <PATH>` — a `PATH` to the Godot project containing the manifest
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
- `-p`, `--path <PATH>` — a `PATH` to the Godot project containing the manifest
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

## **gdpack `update`**

> ❕ **NOTE:** This only applies to `git`-backed dependencies specified with "branch" or "commit" references. A future update may allow this command to locate newer dependency versions.

Update one or more remote addon dependencies to their latest version.

### Usage

`gdpack update [OPTIONS] [NAME]`

### Options

- `-p`, `--path <PATH>` — a `PATH` to the Godot project containing the manifest
- `-t`, `--target <TARGET>` — update dependencies only for `TARGET` (can be specified more than once)

### Arguments

- `[NAME]` — the name of an addon to update (if omitted, all addons are updated)
  - Example values:
    - `godot-next`

## **gdpack `vendor`**

Vendor remote addon dependencies into the specified directory.

### Usage

`gdpack vendor [OPTIONS] <OUT>`

### Options

- `-f`, `--force` — forcibly overwrite existing directories in `OUT`
- `-p`, `--path <PATH>` — a `PATH` to the Godot project containing the manifest
- `--prod`, `--production` — don't vendor development dependencies
- `-t`, `--target <TARGET>` — vendor dependencies only for `TARGET` (can be specified more than once)

### Arguments

- `<OUT>` — the name of a directory to vendor dependencies into; the directory's parent must exist
  - Example values:
    - `../some/path`
