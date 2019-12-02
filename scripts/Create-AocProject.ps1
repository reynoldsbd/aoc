#!/usr/bin/pwsh

<#

.SYNOPSIS
Creates an empty Rust project for an AoC challenge

#>

param(

    # Infer which project to create next
    [Parameter(ParameterSetName = "Next")]
    [switch]
    $Next,

    # Create project for the specified year
    [Parameter(
        Position = 0,
        ParameterSetName = "Specific"
    )]
    [int]
    $Year,

    # Create project for the specified day
    [Parameter(
        Position = 1,
        ParameterSetName = "Specific"
    )]
    [int]
    $Day
)

$ErrorActionPreference = "Stop"

. (Join-Path $PSScriptRoot Common.ps1)

if ($Next) {
    $Year = $CurrentYear
    $Day = $CurrentDay + 1
    if ($Day -gt 25) {
        $Year += 1
        $Day = 1
    }
}

# Scream if the directory already exists

$challengeDir = getChallengeDir $Year $Day

if (Test-Path $challengeDir) {
    Write-Warning "The directory $challengeDir already exists. Overwrite?"
    $reply = Read-Host -Prompt "[y/n]"
    if ($reply -imatch "y") {
        Remove-Item -Recurse $challengeDir
    } else {
        Write-Warning "Aborting."
        exit
    }
}



# Copy project template and replace all occurences of _crate with a unique name

$crateName = getCrateName $Year $Day
Write-Host "Creating $crateName"

Copy-Item -Recurse $PSScriptRoot/template $challengeDir

function setCrateName($file) {
    $content = Get-Content $file | Out-String |
        ForEach-Object { $_ -replace "_crate",$crateName }
    $content > $file
}

setCrateName $challengeDir/Cargo.toml
setCrateName $challengeDir/src/part1.rs
setCrateName $challengeDir/src/part2.rs
