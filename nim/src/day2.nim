import strutils, std/tables

proc transform(opp: string, win: bool): string =
    case opp:
        of "A":
            if win:
                return "B"
            else:
                return "C"
        of "B":
            if win:
                return "C"
            else:
                return "A"
        of "C":
            if win:
                return "A"
            else:
                return "B"

# hehe
proc do_i_wanna_win_or_not(me: string): bool =
    if me == "C":
        return true
    else:
        return false
    

proc solveDay2(input: string): (int, int) =
    var 
        lookup = initTable[string, string]()
        shapeScore = initTable[string, int]()

    lookup["X"] = "A"
    lookup["Y"] = "B"
    lookup["Z"] = "C"

    shapeScore["A"] = 1
    shapeScore["B"] = 2
    shapeScore["C"] = 3

    let parsed = input.split("\n")
    var 
        total = 0
        total2 = 0
    for round in parsed:
        let 
            turns: seq[string] = round.split(' ')
            opp = turns[0]
            me = lookup[turns[1]]
            other_me = if me == "B": opp else: transform(opp, do_i_wanna_win_or_not(me))

        if me == opp:
            total += shapeScore[me] + 3
        elif (me == "A" and opp == "C") or (me == "B" and opp == "A") or (me == "C" and opp == "B"):
            total += shapeScore[me] + 6
        elif (me == "A" and opp == "B") or (me == "B" and opp == "C") or (me == "C" and opp == "A"):
            total += shapeScore[me]

        if other_me == opp:
            total2 += shapeScore[other_me] + 3
        elif (other_me == "A" and opp == "C") or (other_me == "B" and opp == "A") or (other_me == "C" and opp == "B"):
            total2 += shapeScore[other_me] + 6
        elif (other_me == "A" and opp == "B") or (other_me == "B" and opp == "C") or (other_me == "C" and opp == "A"):
            total2 += shapeScore[other_me]

    (total, total2)