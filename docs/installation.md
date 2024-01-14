# **Installation**

The easiest way to install `gdpack` is by using the pre-built binaries. These can be manually downloaded and configured, but automated installation scripts are provided and recommended.

Alternatively, you can install `gdpack` from source using the latest supported version of [Rust](https://www.rust-lang.org/tools/install). See [Install from source](#install-from-source) for more details.

## **Pre-built binaries (recommended)**

> ⚠️ **WARNING:** It's good practice to inspect an installation script prior to execution. The scripts are included in this repository and can be reviewed prior to use.

### **Linux/MacOS**

```sh
curl https://raw.githubusercontent.com/coffeebeats/gdpack/main/scripts/install.sh | sh
```

### **Windows**

#### **Git BASH for Windows**

If you're using [Git BASH for Windows](https://gitforwindows.org/) follow the recommended [Linux/MacOS](#linuxmacos) instructions.

#### **Powershell**

> ❕ **NOTE:** In order to run scripts in PowerShell, the [execution policy](https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.core/about/about_execution_policies) must _not_ be `Restricted`. Consider running the following command
> if you encounter `UnauthorizedAccess` errors when following these instructions. See [Set-ExecutionPolicy](https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.security/set-executionpolicy) documentation for details.
>
> ```sh
> Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope LocalMachine
> ```

```sh
Invoke-WebRequest `
    -UseBasicParsing `
    -Uri "https://raw.githubusercontent.com/coffeebeats/gdpack/main/scripts/install.ps1" `
    -OutFile "./install-gdpack.ps1"; `
    &"./install-gdpack.ps1"; `
    Remove-Item "./install-gdpack.ps1"
```

### **Manual download**

> ❕ **NOTE:** The instructions below provide `bash`-specific commands for a _Linux_-based system. While these won't work in _Powershell_, the process will be similar.

1. Download a prebuilt binary from the corresponding GitHub release. Set `VERSION`, `OS`, and `ARCH` to the desired values.

    ```sh
    VERSION=0.0.0 OS=linux ARCH=x86_64; \
    curl -LO https://github.com/coffeebeats/gdpack/releases/download/v$VERSION/gdpack-$VERSION-$OS-$ARCH.tar.gz
    ```

2. Extract the downloaded archive. To customize the `gdpack` install location, set `GDPACK_HOME` to the desired location (defaults to `$HOME/.gdpack` on Linux/MacOS).

    ```sh
    GDPACK_HOME=$HOME/.gdpack; \
    mkdir -p $GDPACK_HOME/bin && \
    tar -C $GDPACK_HOME/bin -xf gdpack-$VERSION-$OS-$ARCH.tar.gz
    ```

3. Export the `GDPACK_HOME` environment variable and add `$GDPACK_HOME/bin` to `PATH`. Add the following to your shell profile script (e.g. in `.bashrc`, `.zshenv`, `.profile`, or something similar).

    ```sh
    export GDPACK_HOME="$HOME/.gdpack"
    export PATH="$GDPACK_HOME/bin:$PATH"
    ```

## **Install from source**

`gdpack` is a Rust project and can be installed using `cargo build`. This option is not recommended as it requires having the Rust toolchain installed, it's slower than downloading a prebuilt binary, and there may be instability due to using a different version of Rust than it was developed with.

```sh
cargo install --git github.com/coffeebeats/gdpack/cmd/gdpack --tag v0.0.12 # x-release-please-version
```

Once `gdpack` is installed a few things need to be configured. Follow the instructions below based on your operating system.

### **Linux/MacOS**

1. Export the `GDPACK_HOME` environment variable and add `$GDPACK_HOME/bin` to the `PATH` environment variable.

    Add the following to your shell's profile script/RC file:

    ```sh
    export GDPACK_HOME="$HOME/.gdpack"
    export PATH="$GDPACK_HOME/bin:$PATH"
    ```

### **Windows (Powershell)**

1. Export the `GDPACK_HOME` environment variable using the following:

    ```sh
    $GdPackHomePath = "${env:LOCALAPPDATA}\gdpack" # Replace with whichever path you'd like.
    [System.Environment]::SetEnvironmentVariable("GDPACK_HOME", $GdPackHomePath, "User")
    ```

2. Add `$GDPACK_HOME/bin` to your `PATH` environment variable:

    > ❕ **NOTE:** Make sure to restart your terminal after the previous step so that any changes to `$GDPACK_HOME` have been updated.

    ```sh
    $PathParts = [System.Environment]::GetEnvironmentVariable("PATH", "User").Trim(";") -Split ";"
    $PathParts = $PathParts.where{ $_ -ne "${env:GDPACK_HOME}\bin" }
    $PathParts = $PathParts + "${env:GDPACK_HOME}\bin"

    [System.Environment]::SetEnvironmentVariable("PATH", $($PathParts -Join ";"), "User")
    ```
