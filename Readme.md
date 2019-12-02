This repository contains my personal solutions for the Advent of Code
challenges, as well as some scripts that are handy for scaffolding and running
new challenges.


# Running a Challenge

Use */scripts/Invoke-AocProject.ps1* to build and run a solution. By default,
the solution to the most recent challenge is run. This can be overridden using
the `Year` and `Day` parameters.

Each solution is broken up into two binaries, *part1* and *part2*, which
correspond to the first and second parts of each AoC challenge. By default,
*Invoke-AocProject.ps1* will run *part1*. This can be overridden using the
`Part` parameter.


# Scaffolding a New Challenge

Use */scripts/Create-AocProject.ps1* to create a skeleton project for a new
challenge. There are two ways to use this script:

When used with `-Next`, this script will infer the "next" AoC challenge and
create the corresponding directory. For example, if the */2019/* directory
contains only *01/* and *02/* subdirectories (corresponding respectively to the
Day 1 and Day 2 challenges from AoC 2019), then this script will create a new
skeleton project under */2019/03*.

Alternatively, you can manually specify which challenge to scaffold using the
`Year` and `Day` parameters. If a solution already exists for the specified day,
a warning is issued and you will have the option to overwrite that solution.

In any case, the newly created project will be added to this repository's Cargo
workspace, making it easy and efficient to re-use code across solutions.
