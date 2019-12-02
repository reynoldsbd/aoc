#!/usr/bin/pwsh

<#

.SYNOPSIS
Builds and runs the Rust project for an AoC challenge

#>

param(

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
    $Day,

    # Whether to build and run part 1 or 2
    [Parameter(
        Position = 3
    )]
    [ValidateSet(
        "part1",
        "part2"
    )]
    [string]
    $Part = "part1"
)

. (Join-Path $PSScriptRoot Common.ps1)

if (!$Year) {
    $Year = $CurrentYear
}

if (!$Day) {
    $Day = $CurrentDay
}

$challengeDir = getChallengeDir $Year $Day

Push-Location $challengeDir
cargo run --bin $Part
Pop-Location
