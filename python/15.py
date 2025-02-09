import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import heapq
from intcode import Intcode


def part1(data):
    """ 2019 Day 15 Part 1
    """

    intcode = Intcode({i: int(x) for i, x in enumerate(data[0].split(','))})
    area = handler(intcode, False)

    for (k, v) in zip(area.keys(), area.values()):
        if v == 2:
            end = [int(x) for x in k.split(',')]
            break

    return aStar(area, [0, 0], end)


def part2(data):
    """ 2019 Day 15 Part 2
    """

    intcode = Intcode({i: int(x) for i, x in enumerate(data[0].split(','))})
    area = handler(intcode, False)

    for (k, v) in zip(area.keys(), area.values()):
        if v == 2:
            end = [int(x) for x in k.split(',')]
            break

    aStar(area, [0, 0], end)

    maximum = float('-inf')
    for (k, v) in zip(area.keys(), area.values()):
        if v == 0:
            continue
        
        dist = aStar(area, end, [int(x) for x in k.split(',')])

        if dist > maximum:
            maximum = dist

    return maximum


def handler(intcode: Intcode, verbose):
    dirs = [[0, 1], [0, -1], [-1, 0], [1, 0]]
    moveOrder = [1, 4, 2, 3]
    loc = [0, 0]
    area = {}
    area[arrToStr(loc)] = 1

    while loc != [0, 0] or len(area) == 1:
        while True:
            intcode.addInput(moveOrder[0])
            intcode.runCode()
            outputs = intcode.getOutput()
            intcode.resetOutput()

            newLoc = [l + m for (l, m) in zip(loc, dirs[moveOrder[0] - 1])]
            area[arrToStr(newLoc)] = outputs[0]

            if outputs[0] != 0:
                loc = newLoc[:]
                for _ in range(3):
                    moveOrder.append(moveOrder.pop(0))
                break

            moveOrder.append(moveOrder.pop(0))

    if verbose:
        printSpace(area, loc)

    return area


def arrToStr(arr):
    string = str(arr[0])
    for a in arr[1:]:
        string += ',' + str(a)

    return string


def printSpace(area, loc):
    maxs = [float('-inf')] * 2
    mins = [float('inf')] * 2

    for lStr in area:
        l = [int(x) for x in lStr.split(',')]

        for (i, c) in enumerate(l):
            if c < mins[i]:
                mins[i] = c
            if c > maxs[i]:
                maxs[i] = c

    for y in range(maxs[1], mins[1] - 1, -1):
        for x in range(mins[0], maxs[0] + 1):
            locStr = str(x) + ',' + str(y)
            l = area[locStr] if locStr in area else 0

            c = 'D' if x == loc[0] and y == loc[1] else ('#' if l == 0 else (' ' if l == 1 else '!'))
            print(c,end='')

        print('')

    print('-'*(maxs[0] - mins[0] + 1))


def heuristic(start, end):
    return sum([abs(e - s) for (s, e) in zip(start, end)])


def getNext(curr, area):
    dirs = [[0, 1], [0, -1], [-1, 0], [1, 0]]

    adj = []
    for d in dirs:
        adj.append(arrToStr([l + m for (l, m) in zip(curr, d)]))

    for i in range(len(adj) - 1, -1, -1):
        a = adj[i]
        if a not in area or area[a] == 0:
            adj.pop(i)

    return adj


def aStar(area, start, end):
    openList_heap = [[heuristic(start, end), 0, start]]
    closedList = {}
    heuristics = {}

    heuristics[arrToStr(start)] = heuristic(start, end)

    heapq.heapify(openList_heap)
    while len(openList_heap) != 0:
        qF, qG, q = heapq.heappop(openList_heap)  
        
        if q == end:
            return qG

        nextStates = getNext(q, area)

        for n in nextStates:
            state = [int(x) for x in n.split(',')]
            nG = qG + 1

            if n in heuristics:
                nH = heuristics[n]
            else:
                nH = heuristic(state, end)
                heuristics[n] = nH

            nF = nG + nH

            found = False
            for item in openList_heap:
                if item[2] == state and item[0] <= nF:
                    found = True
                    break

            if found or (n in closedList and closedList[n][0] <= nF):
                continue

            heapq.heappush(openList_heap, [nF, nG, state])

        closedList[arrToStr(q)] = [qF, qG, q]


def main(input_path: Optional[Path | str]=None, verbose: bool=False) -> Tuple[Tuple[Any, float]]:
    if not input_path:
        if not (input_path := sys.argv[1] if len(sys.argv) > 1 else None):
            year, day = re.findall(r'\d+', str(__file__))[-2:]
            input_path = Path(Path(__file__).parent.parent.parent, "Inputs", f"{year}_{day}.txt")
    
    with open(input_path, encoding='UTF-8') as f:
        data = [line.strip('\n') for line in f.readlines()]

    with Timer() as p1_time:
        p1 = part1(data)

    if verbose:
        print(f"\nPart 1:\nFewest steps to oxygen system: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nMinutes to fill with oxygen: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)