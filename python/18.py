import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import heapq
from collections import defaultdict, deque
from itertools import product


def part1(data):
    """ 2019 Day 18 Part 1

    >>> part1([ '#########',\
                '#b.A.@.a#',\
                '#########'])
    8
    >>> part1([ '#################',\
                '#i.G..c...e..H.p#',\
                '########.########',\
                '#j.A..b...f..D.o#',\
                '########@########',\
                '#k.E..a...g..B.n#',\
                '########.########',\
                '#l.F..d...h..C.m#',\
                '#################'])
    136
    """

    keys = {}
    doors = {}
    paths = set()
    for y, line in enumerate(data):
        for x, l in enumerate(line):
            if l == '#':
                continue

            paths.add((x, y))

            if l == '@':
                start = (x, y)

            if ord('a') <= ord(l) <= ord('z'):
                keys[(x, y)] = l
            elif ord('A') <= ord(l) <= ord('Z'):
                doors[(x, y)] = l

    nodes = defaultdict(lambda: defaultdict(lambda: (float('inf'), 0)))
    for node in list(keys.keys()) + [start]:
        openList = deque([[0, node, 0]])
        visited = set()

        while len(openList):
            q, pos, keysNeeded = openList.popleft()
            if pos in keys:
                nodes[node][pos] = (q, keysNeeded)

                if pos != node:
                    continue

            visited.add(pos)
            for nPos in [tuple(p + o for p, o in zip(pos, offset)) for offset in [(1, 0), (-1, 0), (0, 1), (0, -1)]]:
                if nPos in paths and nPos not in visited:
                    if nPos in doors:
                        openList.append([q + 1, nPos, keysNeeded | (1 << (ord(doors[nPos]) - ord('A')))])
                    else:
                        openList.append([q + 1, nPos, keysNeeded])

    for k, i, j in product(nodes.keys(), repeat = 3):
        if nodes[i][j][0] > nodes[i][k][0] + nodes[k][j][0]:
            nodes[i][j] = (nodes[i][k][0] + nodes[k][j][0], nodes[i][k][1] | nodes[k][j][1])

    for k, connections in nodes.items():
        for n, (d, _) in list(connections.items()):
            if d == float('inf'):
                del(connections[n])

    openList = [[0, start, 0]]
    openDict = defaultdict(dict)
    openDict[start][0] = 0
    visited = defaultdict(dict)
    endKeys = sum(1 << (ord(k) - ord('a')) for k in keys.values())

    while len(openList):
        pathLen, pos, keysFound = heapq.heappop(openList)

        if keysFound == endKeys:
            return pathLen
        
        if keysFound in openDict[pos]:
            del(openDict[pos][keysFound])
        
        if keysFound in visited[pos] and visited[pos][keysFound] <= pathLen:
            continue
        
        visited[pos][keysFound] = pathLen
        
        for nPos, (q, keysNeeded) in nodes[pos].items():
            if not (keysFound | keysNeeded) == keysFound:
                continue
            
            newKeysFound = keysFound | (1 << (ord(keys[nPos]) - ord('a')))
            if newKeysFound in visited[nPos] and visited[nPos][newKeysFound] <= pathLen + q:
                continue

            if newKeysFound in openDict[nPos] and openDict[nPos][newKeysFound] <= pathLen + q:
                continue
            
            openDict[nPos][newKeysFound] = pathLen + q
            heapq.heappush(openList, [pathLen + q, nPos, newKeysFound])

    return -1


def part2(data):
    """ 2019 Day 18 Part 2

    >>> part2([ '#############',\
                '#DcBa.#.GhKl#',\
                '#.###...#I###',\
                '#e#d#.@.#j#k#',\
                '###C#...###J#',\
                '#fEbA.#.FgHi#',\
                '#############'])
    32
    """

    lines = [list(line) for line in data]

    for (y, line) in enumerate(lines):
        for (x, l) in enumerate(line):
            if l == '.':
                deadEndFill(lines, [x, y])
    POIs = {}
    keys = {}
    startCount = 1
    for (y, line) in enumerate(lines):
        for (x, l) in enumerate(line):
            if ord('a') <= ord(l.lower()) <= ord('z'):
                POIs[l] = pointOfInterest([x, y], l)

            if ord('a') <= ord(l) <= ord('z'):
                keys[l] = [x, y]

            pos = [x, y]
            for offset in [[x1, y1] for x1 in range(-1, 2) for y1 in range(-1, 2)]:
                newPos = [p + o for p, o in zip(pos, offset)]
                try:
                    if data[newPos[1]][newPos[0]] == '@':
                        if sum([abs(n) for n in offset]) == 2:
                            lines[y][x] = str(startCount)
                            keys[str(startCount)] = [x, y]
                            POIs[str(startCount)] = pointOfInterest([x, y], str(startCount))
                            startCount += 1
                        else:
                            lines[y][x] = '#'
                except IndexError:
                    continue

    for p in POIs.values():
        p.genNeighbors(lines, POIs)

    pathLen, collected = collectKeysP2(POIs, keys)
    collected = collected.replace('@', '')

    return pathLen    


class pointOfInterest:
    def __init__(self, pos, c):
        self.pos = pos[:]
        self.id = c
        self.neighbors = []

    def genNeighbors(self, lines, others):
        openList = [[self.pos]]
        closedList = []

        while len(openList) != 0:
            path = openList.pop(0)
            pos = path[-1]

            for n in [[p + o for (p, o) in zip(pos, offset)] for offset in [[1, 0], [-1, 0], [0, 1], [0, -1]]]:
                if n in closedList or lines[n[1]][n[0]] == '#':
                    continue

                if lines[n[1]][n[0]] == '.' or lines[n[1]][n[0]] == '@' or ord('1') <= ord(lines[n[1]][n[0]])  <= ord('4'):
                    openList.append(path + [n])
                else:
                    self.neighbors.append([others[lines[n[1]][n[0]]], len(path)])

            closedList.append(pos)


class Path:
    def __init__(self, end, length):
        self.end = end
        self.length = length


def deadEndFill(lines, deadEnd):
    if lines[deadEnd[1]][deadEnd[0]] != '.':
        return

    possible = []
    
    for n in [[p + o for p, o in zip(deadEnd, offset)] for offset in [[1, 0], [-1, 0], [0, 1], [0, -1]]]:
        if 0 <= n[0] < len(lines[0]) and 0 <= n[1] < len(lines):
            if lines[n[1]][n[0]] != '#':
                possible.append(n)

    if len(possible) == 1:
        lines[deadEnd[1]][deadEnd[0]] = '#'
        deadEndFill(lines, possible[0])


def genPaths(start, neededKeys):
    openList = [[start, 0]]
    closedList = []

    paths = []
    while len(openList) != 0:
        currPOI, pathLen = openList.pop(0)

        for neighbor, dist in currPOI.neighbors:
            if neighbor in closedList or neighbor.id in neededKeys.upper():
                continue

            if neighbor.id in neededKeys:
                paths.append(Path(neighbor.id, pathLen + dist))
            else:
                openList.append([neighbor, pathLen + dist])

        closedList.append(currPOI)

    return paths


def collectKeysP1(POIs, allKeys):
    openList = [[0, "".join(sorted([k for k in allKeys.keys() if k != '@'])), '@']]
    closedList = {}

    paths = {k: {} for k in allKeys.keys()}

    while len(openList) != 0:
        pathLen, neededKeys, collected = heapq.heappop(openList)
        currKey = collected[-1]

        if len(neededKeys) == 0:
            return [pathLen, collected]

        if neededKeys not in paths[currKey].keys():
            paths[currKey][neededKeys] = genPaths(POIs[currKey], neededKeys)

        for p in paths[currKey][neededKeys]:
            newPathLen = pathLen + p.length
            newNeeded = neededKeys[:].replace(p.end, '')
            newCollected = collected[:] + p.end

            if newNeeded in closedList and closedList[newNeeded] <= newPathLen:
                continue

            valid = True
            for other in openList:
                if other[0] <= newPathLen and other[1] == newNeeded and other[2][-1] == newCollected[-1]:
                    valid = False
                    break

            if valid:
                heapq.heappush(openList, [newPathLen, newNeeded, newCollected])

        closedList[neededKeys] = pathLen

    return [0, 'ERROR']


def collectKeysP2(POIs, allKeys):
    openList = [[0, "".join(sorted([k for k in allKeys.keys() if k != '@' and not (ord('1') <= ord(k) <= ord('4'))])), '', ['1', '2', '3', '4']]]
    closedList = {}

    paths = {k: {} for k in allKeys.keys()}

    while len(openList) != 0:
        pathLen, neededKeys, collected, currKeys = heapq.heappop(openList)

        if len(neededKeys) == 0:
            return [pathLen, collected]

        for (i, currKey) in enumerate(currKeys):
            if neededKeys not in paths[currKey].keys():
                paths[currKey][neededKeys] = genPaths(POIs[currKey], neededKeys)

            for p in paths[currKey][neededKeys]:
                newPathLen = pathLen + p.length
                newNeeded = neededKeys[:].replace(p.end, '')
                newCollected = collected[:] + p.end
                newCurrKeys = currKeys[:]
                newCurrKeys[i] = p.end

                if newNeeded in closedList and closedList[newNeeded] <= newPathLen:
                    continue

                valid = True
                for other in openList:
                    if other[0] <= newPathLen and other[1] == newNeeded and other[3] == newCurrKeys:
                        valid = False
                        break

                if valid:
                    heapq.heappush(openList, [newPathLen, newNeeded, newCollected, newCurrKeys])

        closedList[neededKeys] = pathLen

    return [0, 'ERROR']


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
        print(f"\nPart 1:\nShortest Path: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nShortest Path: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)