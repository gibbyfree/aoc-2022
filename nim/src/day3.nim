import strutils, tables, sequtils, sugar


proc find_common(first: string, second: string): char =
    var score_set = initTable[char, int]()
    first.toSeq().map()


proc solveDay3(input: string): (int, int) =
    let priority = "0abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
    let sol1 = input.split("\n")
        .map(proc (line: string): int =
            let first = line[0 .. line.len() / 2]
            let second = line[line.len() / 2 .. line.len()]

        })


    (0, 0)