import strutils, sequtils, math, std/sets

proc find_common(line: string): HashSet[char] =
    let 
        mid = int(line.len / 2)
        first = line[0 .. mid - 1]
        second = line[mid .. line.len() - 1]
    
    var 
        first_set = toHashSet(first)
        second_set = toHashSet(second)

    return (first_set * second_set)

proc find_common_badge(first: string, second: string, third: string): HashSet[char] =
    var
        first = toHashSet(first)
        second = toHashSet(second)
        third = toHashSet(third)

    return (first * second * third)

proc solveDay3(input: string): (int, int) =
    let priority = "0abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
    let sol1 = input.split("\n")
        .map(proc (line: string): HashSet[char] =
            find_common(line)
        ).map(proc (s: HashSet[char]): int =
            var sum = 0
            for item in s: 
                sum += priority.find(item)
            return sum
        ).sum()

    let lines = input.split("\n")
    var 
        sol2 = 0
        chunk: seq[string]

    for line in lines:
        chunk.add(line)
        if chunk.len == 3:
            var common = find_common_badge(chunk[0], chunk[1], chunk[2])
            sol2 += priority.find(common.pop)
            chunk.setLen(0)

    (sol1, sol2)