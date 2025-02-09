import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
def part1(data):
    """ 2019 Day 3 Part 1

    >>> part1(['R75,D30,R83,U83,L12,D49,R71,U7,L72', 'U62,R66,U55,R34,D71,R55,D58,R83'])
    159
    >>> part1(['R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51', 'U98,R91,D20,R16,D67,R40,U7,R15,U6,R7'])
    135
    """

    wires = [wire(line) for line in data]

    intersections = findIntersections(wires)
    manhatDists = []
    for intersection in intersections:
        manhatDists.append(manhatDist(intersection, [0, 0]))

    return min(manhatDists)


def part2(data):
    """ 2019 Day 3 Part 2

    >>> part2(['R75,D30,R83,U83,L12,D49,R71,U7,L72', 'U62,R66,U55,R34,D71,R55,D58,R83'])
    610
    >>> part2(['R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51', 'U98,R91,D20,R16,D67,R40,U7,R15,U6,R7'])
    410
    """

    wires = [wire(line) for line in data]

    intersections = findIntersections(wires)

    delays = [0] * len(intersections)
    for w in wires:
        delay = 0
        point = [0, 0]
        direction = w.path[0][0]
        nextIndex = 1
        encountered = [False] * len(delays)

        while point != w.corners[-1]:
            nextCorner = w.corners[nextIndex]
            if point == nextCorner:
                direction = w.path[nextIndex][0]
                nextIndex += 1

            if point in intersections:
                i = intersections.index(point)
                if not encountered[i]:
                    delays[i] += delay
                    encountered[i] = True

            move = [1 if direction == 'R' else (-1 if direction == 'L' else 0), 1 if direction == 'U' else (-1 if direction == 'D' else 0)]
            point = [item1 + item2 for (item1, item2) in zip(point, move)]
            delay += 1

    return min(delays)


class wire:
    def __init__(self, path):
        self.path = path[:]
        self.path = self.path.split(',')
        self.corners = [[0, 0]]
        for i in path.split(','):
            if i[0] == 'U':
                self.corners.append([item1 + item2 for (item1, item2) in zip(self.corners[-1], [0, int(i[1:])])])
            elif i[0] == 'D':
                self.corners.append([item1 - item2 for (item1, item2) in zip(self.corners[-1], [0, int(i[1:])])])
            elif i[0] == 'L':
                self.corners.append([item1 - item2 for (item1, item2) in zip(self.corners[-1], [int(i[1:]), 0])])
            else:
                self.corners.append([item1 + item2 for (item1, item2) in zip(self.corners[-1], [int(i[1:]), 0])])


class lineSeg:
    def __init__(self, p):
        if p[0][0] == p[1][0]:
            self.horiz = True
            self.vert = False
            self.same = p[0][0]
            self.ends = [p[0][1], p[1][1]]
        elif p[0][1] == p[1][1]:
            self.horiz = False
            self.vert = True
            self.same = p[0][1]
            self.ends = [p[0][0], p[1][0]]

        self.ends.sort()


def manhatDist(p1, p2):
    dist = 0
    for (l1, l2) in zip(p1, p2):
        dist += abs(l1 - l2)

    return dist


def findIntersections(wires):
    intersections = []
    for i in range(len(wires[0].corners) - 1):
        line1 = lineSeg([wires[0].corners[i], wires[0].corners[i + 1]])
        for j in range(len(wires[1].corners) - 1):
            line2 = lineSeg([wires[1].corners[j], wires[1].corners[j + 1]])
            if line1.vert != line2.vert and line2.ends[0] <= line1.same <= line2.ends[1] and line1.ends[0] <= line2.same <= line1.ends[1]:
                intersection = [line1.same if line1.horiz else line2.same, line1.same if line1.vert else line2.same]
                if intersection != [0, 0]:
                    intersections.append(intersection)

    return intersections


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
        print(f"\nPart 1:\n{p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\n{p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)