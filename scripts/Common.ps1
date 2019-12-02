# Common logic and values shared by scripts

$RepoRoot = Split-Path -Parent $PSScriptRoot

$CurrentYear = Get-ChildItem -Path $RepoRoot |
    Where-Object Name -Match "\d{4}" |
    Measure-Object -Property Name -Maximum |
    Select-Object -ExpandProperty Maximum
$CurrentYear = [int]$CurrentYear

$CurrentDay = Get-ChildItem -Path $RepoRoot/$CurrentYear |
    Where-Object Name -Match "\d{2}" |
    Measure-Object -Property Name -Maximum |
    Select-Object -ExpandProperty Maximum
$CurrentDay = [int]$CurrentDay

function getChallengeDir($year, $day) {
    "$RepoRoot/$year/{0:D2}" -f $day
}

function getCrateName($year, $day) {
    "aoc_${year}_{0:D2}" -f $day
}
