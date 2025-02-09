import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import heapq


def part1(data):
    """ 2019 Day 20 Part 1
    """

    portals = genPortals(data)

    for portal in portals:
        if portal.id == 'AA':
            openList = [[0, portal]]

    closedList = {}

    while len(openList) != 0:
        pathLen, curr = heapq.heappop(openList)

        if curr.id == 'ZZ':
            return pathLen

        for newCurr, newLen in curr.neighbors:
            newLen += pathLen

            valid = True
            for otherLen, otherCurr in openList:
                if otherLen <= newLen and otherCurr == newCurr:
                    valid = False
                    break
                    
            if not valid:
                continue
            
            if newCurr.dictString() in closedList and closedList[newCurr.dictString()] <= newLen:
                continue

            heapq.heappush(openList, [newLen, newCurr])

        if curr.connect != 0:
            newCurr = curr.connect
            newLen = pathLen + 1

            valid = True
            for otherLen, otherCurr in openList:
                if otherLen <= newLen and otherCurr == newCurr:
                    valid = False
                    break
                    
            if valid and not (newCurr.dictString() in closedList and closedList[newCurr.dictString()] <= newLen):
                heapq.heappush(openList, [newLen, newCurr])

        closedList[curr.dictString()] = pathLen

    return -1


def part2(data):
    """ 2019 Day 20 Part 2
    """

    portals = genPortals(data)

    for portal in portals:
        if portal.id == 'AA':
            openList = [[0, 0, portal]]

    closedList = {}

    while len(openList) != 0:
        pathLen, level, curr = heapq.heappop(openList)
        
        if level != 0 and (curr.id == 'AA' or curr.id == 'ZZ'):
            continue

        if curr.id == 'ZZ':
            return pathLen

        for newCurr, newLen in curr.neighbors:
            newLen += pathLen
            newLevel = level

            valid = True
            for otherLen, otherLevel, otherCurr in openList:
                if otherLen <= newLen and otherLevel == newLevel and otherCurr == newCurr:
                    valid = False
                    break
                    
            if not valid:
                continue
            
            if newCurr.dictString() + str(newLevel) in closedList and closedList[newCurr.dictString() + str(newLevel)] <= newLen:
                continue

            heapq.heappush(openList, [newLen, newLevel, newCurr])

        if curr.connect != 0:
            newCurr = curr.connect
            newLen = pathLen + 1
            if curr.pNum == 1:
                newLevel = level - 1
            else:
                newLevel = level + 1

            valid = newLevel >= 0
            for otherLen, otherLevel, otherCurr in openList:
                if otherLen <= newLen and otherLevel == newLevel and otherCurr == newCurr:
                    valid = False
                    break
                    
            if valid and not (newCurr.dictString() + str(newLevel) in closedList and closedList[newCurr.dictString() + str(newLevel)] <= newLen):
                heapq.heappush(openList, [newLen, newLevel, newCurr])

        closedList[curr.dictString() + str(level)] = pathLen

    return -1


class Portal:
    def __init__(self, possible, lines, c):
        self.pos = [p[:] for p in possible if 0 <= p[0] < len(lines[0]) and 0 <= p[1] < len(lines) and lines[p[1]][p[0]] == '.'][0]
        self.id = c
        self.neighbors = []
        self.connect = 0
        self.pNum = 1 if 2 in self.pos or len(lines) - 3 in self.pos or len(lines[0]) - 3 in self.pos else 2

    def __lt__(self, _):
        return False

    def genNeighbors(self, lines, others):
        for o in others:
            if self != o and self.id == o.id:
                self.connect = o

        openList = [[self.pos]]
        closedList = []

        while len(openList) != 0:
            path = openList.pop(0)
            pos = path[-1]

            for n in [[p + o for p, o in zip(pos, offset)] for offset in [[1, 0], [-1, 0], [0, 1], [0, -1]]]:
                if n in closedList or lines[n[1]][n[0]] == '#':
                    continue

                if lines[n[1]][n[0]] == '.':
                    openList.append(path + [n])
                elif lines[n[1]][n[0]] != self:
                    self.neighbors.append([lines[n[1]][n[0]], len(path) - 1])

            closedList.append(pos)

    def dictString(self):
        return self.id + str(self.pNum)
    

def genPortals(data):
    lines = [list(line) for line in data]

    for (y, line) in enumerate(lines):
        for (x, l) in enumerate(line):
            if l != '#':
                deadEndFill(lines, [x, y])

    portals = []
    for (y, line) in enumerate(lines[:-1]):
        for (x, l) in enumerate(line[:-1]):
            if ord('A') <= ord(l) <= ord('Z'):
                c1 = lines[y][x + 1]
                c2 = lines[y + 1][x]

                if ord('A') <= ord(c1) <= ord('Z'):
                    portals.append(Portal([[x + 2, y], [x - 1, y]], lines, "".join([l, c1])))
                elif ord('A') <= ord(c2) <= ord('Z'):
                    portals.append(Portal([[x, y + 2], [x, y - 1]], lines, "".join([l, c2])))

    for portal in portals:
        for offset in [[1, 0], [-1, 0], [0, 1], [0, -1]]:
            n = [p + o for p, o in zip(portal.pos, offset)]
            if ord('A') <= ord(lines[n[1]][n[0]]) <= ord('Z'):
                lines[n[1]][n[0]] = portal
                n1 = [p + o for p, o in zip(n, offset)]
                lines[n1[1]][n1[0]] = ' '
                break

    for portal in portals:
        portal.genNeighbors(lines, portals)

    return portals
    

def deadEndFill(lines, deadEnd):
    possible = []
    
    for n in [[p + o for p, o in zip(deadEnd, offset)] for offset in [[1, 0], [-1, 0], [0, 1], [0, -1]]]:
        if 0 <= n[0] < len(lines[0]) and 0 <= n[1] < len(lines):
            if lines[n[1]][n[0]] != '#':
                possible.append(n)

    if len(possible) == 1:
        lines[deadEnd[1]][deadEnd[0]] = '#'
        deadEndFill(lines, possible[0])


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
        print(f"\nPart 1:\nShortest Path from AA to ZZ: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nShortest Path from AA to ZZ: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)