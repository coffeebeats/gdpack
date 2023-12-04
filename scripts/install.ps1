# This script installs 'gdpack' by downloading prebuilt binaries from the
# project's GitHub releases page. By default the latest version is installed,
# but a different release can be used instead by setting $GDPACK_VERSION.
#
# The script will set up a 'gdpack' cache at '%LOCALAPPDATA%/gdpack'. This
# behavior can be customized by setting '$GDPACK_HOME' prior to running the
# script. Existing Godot artifacts cached in a 'gdpack' store won't be lost, but
# this script will overwrite any 'gdpack' binary artifacts in '$GDPACK_HOME/bin'.
#
# NOTE: Unlike the 'install.sh' counterpart, this script exclusively installs
# the 'gdpack' binary for 64-bit Windows. If an alternative 'gdpack' binary is
# required, follow the documentation for an alternative means of installation:
# https://github.com/coffeebeats/gdpack/blob/v0.0.6/docs/installation.md # x-release-please-version

<#
.SYNOPSIS
  Install 'gdpack' for managing Godot addons.

.DESCRIPTION
  This script downloads the specified version of 'gdpack' from GitHub, extracts
  its artifacts to the 'gdpack' store ('$GDPACK_HOME' or a default path), and then
  updates environment variables as needed.

.PARAMETER NoModifyPath
  Do not modify the $PATH environment variable.

.PARAMETER Version
  Install the specified version of 'gdpack'.

.INPUTS
  None

.OUTPUTS
  $env:GDPACK_HOME\bin\gdpack.exe

.NOTES
  Version:        0.0.6 # x-release-please-version
  Author:         https://github.com/coffeebeats

.LINK
  https://github.com/coffeebeats/gdpack
#>

# ------------------------------ Define: Params ------------------------------ #

Param (
  # NoModifyPath - if set, the user's $PATH variable won't be updated
  [Switch] $NoModifyPath = $False,

  # Version - override the specific version of 'gdpack' to install
  [String] $Version = "v0.0.6" # x-release-please-version
)

# ------------------------- Function: Get-GdPackHome ------------------------- #

# Returns the current value of the 'GDPACK_HOME' environment variable or a
# default if unset.
Function Get-GdPackHome() {
  if ([string]::IsNullOrEmpty($env:GDPACK_HOME)) {
    return Join-Path -Path $env:LOCALAPPDATA -ChildPath "gdpack"
  }

  return $env:GDPACK_HOME
}

# ------------------------ Function: Get-GdPackVersion ----------------------- #

Function Get-GdPackVersion() {
  return "v" + $Version.TrimStart("v")
}

# --------------------- Function: Create-Temporary-Folder -------------------- #

# Creates a new temporary directory for downloading and extracting 'gdpack'. The
# returned directory path will have a randomized suffix.
Function New-TemporaryFolder() {
  # Make a new temporary folder with a randomized suffix.
  return New-Item `
    -ItemType Directory `
    -Name "gdpack-$([System.IO.Path]::GetFileNameWithoutExtension([System.IO.Path]::GetRandomFileName()))"`
    -Path $env:temp
}

# ------------------------------- Define: Store ------------------------------ #

$GdPackHome = Get-GdPackHome

Write-Host "info: setting 'GDPACK_HOME' environment variable: ${GdPackHome}"

[System.Environment]::SetEnvironmentVariable("GDPACK_HOME", $GdPackHome, "User")

# ------------------------------ Define: Version ----------------------------- #
  
$GdPackVersion = Get-GdPackVersion

$GdPackArchive = "gdpack-${GdPackVersion}-windows-x86_64.zip"

# ----------------------------- Execute: Install ----------------------------- #
  
$GdPackRepositoryURL = "https://github.com/coffeebeats/gdpack"

# Install downloads 'gdpack' and extracts its binaries into the store. It also
# updates environment variables as needed.
Function Install() {
  $GdPackTempFolder = New-TemporaryFolder

  $GdPackArchiveURL = "${GdPackRepositoryURL}/releases/download/${GdPackVersion}/${GdPackArchive}"
  $GdPackDownloadTo = Join-Path -Path $GdPackTempFolder -ChildPath $GdPackArchive

  $GdPackHomeBinPath = Join-Path -Path $GdPackHome -ChildPath "bin"

  try {
    Write-Host "info: installing version: '${GdPackVersion}'"

    Invoke-WebRequest -URI $GdPackArchiveURL -OutFile $GdPackDownloadTo

    Microsoft.PowerShell.Archive\Expand-Archive `
      -Force `
      -Path $GdPackDownloadTo `
      -DestinationPath $GdPackHomeBinPath
  
    if (!($NoModifyPath)) {
      $PathParts = [System.Environment]::GetEnvironmentVariable("PATH", "User").Trim(";") -Split ";"
      $PathParts = $PathParts.where{ $_ -ne $GdPackHomeBinPath }
      $PathParts = $PathParts + $GdPackHomeBinPath

      Write-Host "info: updating 'PATH' environment variable: ${GdPackHomeBinPath}"

      [System.Environment]::SetEnvironmentVariable("PATH", $($PathParts -Join ";"), "User")
    }

    Write-Host "info: sucessfully installed executables:`n"
    Write-Host "  gdpack.exe: $(Join-Path -Path $GdPackHomeBinPath -ChildPath "gdpack.exe")"
  }
  catch {
    Write-Host "error: failed to install 'gdpack': ${_}"
  }
  finally {
    Write-Host "`ninfo: cleaning up downloads: ${GdPackTempFolder}"

    Remove-Item -Recurse $GdPackTempFolder
  }
}

Install
