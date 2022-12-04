import os, times, strutils
include src/day1, src/day2, src/day3

template benchmark(name: string, code: untyped) =
    block:
        let t0 = cpuTime()
        code
        let elapsed = (cpuTime() - t0) * 1000
        let elapsedStr = elapsed.formatFloat(format = ffDecimal, precision = 4)
        echo "CPU Runtime [", name, "] ", elapsedStr, "ms"

let
    args = commandLineParams()
    day = args[0]
    nameStr = "Advent of Code 2022 Day " & day

# might want to parameterize here a bit
proc getInput(): string =
    let fileName = "/home/gib/aoc/input/" & day & ".in"
    readFile(fileName).strip()

proc callSolver(x: int, input: string): (int, int) =
    case x:
        of 1:
            solveDay1(input)
        of 2: 
            solveDay2(input)
        of 3:
            solveDay3(input)
        else:
            (0, 0)

benchmark nameStr:
    let (sol1, sol2) = callSolver(parseInt(day), getInput())
    echo "Part 1: ", $sol1, "\nPart 2: ", $sol2
