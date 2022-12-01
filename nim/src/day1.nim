import strutils, sequtils, math, algorithm

proc solveDay1(input: string): (int, int) =
    let parsed: seq[int] = input.split("\n\n")
        .map(proc (chunk: string): int =
            chunk.splitLines()
            .map(proc (line: string): int =
                parseInt(line)
            ).sum() # sum for each elf
        ).sorted()

    let idx = parsed.len - 1
    (parsed[idx], parsed[idx]+parsed[idx-1]+parsed[idx-2])
    