import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import math


def part1(data):
    """ 2019 Day 10 Part 1

    >>> part1(['.#..#', '.....', '#####', '....#', '...##'])
    8
    >>> part1(['.#..##.###...#######', '##.############..##.', '.#.######.########.#', '.###.#######.####.#.', '#####.##.#.##.###.##', '..#####..#.#########', '####################', '#.####....###.#.#.##', '##.#################', '#####.##.###..####..', '..######..##.#######', '####.##.####...##..#', '.#####..#.######.###', '##...#.##########...', '#.##########.#######', '.####.#.###.###.#.##', '....##.##.###..#####', '.#.#.###########.###', '#.#.#.#####.####.###', '###.##.####.##.#..##'])
    210
    """

    asteroids = []
    for (y, line) in enumerate(data):
        for (x, c) in enumerate(line):
            if c == '.':
                continue

            asteroids.append(asteroid([x,y]))

    for a in asteroids:
        a.findVisible(asteroids)

    asteroids.sort(key=visibleSort)
    return len(asteroids[-1].sees)


def part2(data):
    """ 2019 Day 10 Part 2

    >>> part2(['.#..##.###...#######', '##.############..##.', '.#.######.########.#', '.###.#######.####.#.', '#####.##.#.##.###.##', '..#####..#.#########', '####################', '#.####....###.#.#.##', '##.#################', '#####.##.###..####..', '..######..##.#######', '####.##.####...##..#', '.#####..#.######.###', '##...#.##########...', '#.##########.#######', '.####.#.###.###.#.##', '....##.##.###..#####', '.#.#.###########.###', '#.#.#.#####.####.###', '###.##.####.##.#..##'])
    802
    """

    asteroids = []
    for (y, line) in enumerate(data):
        for (x, c) in enumerate(line):
            if c == '.':
                continue

            asteroids.append(asteroid([x,y]))

    for a in asteroids:
        a.findVisible(asteroids)

    asteroids.sort(key=visibleSort)

    station = asteroids.pop(-1)
    removed = []
    while len(removed) + len(station.sees) < 200:
        for r in station.sees:
            a = station.sees[r]
            removed.append(asteroids.pop(asteroids.index(a)))

        station.sees = {}
        station.findVisible(asteroids)

    finalIndex = 199 - len(removed)
    removed = []
    for r in station.sees:
        a = station.sees[r]
        a.findBearing(station)
        removed.append(asteroids.pop(asteroids.index(a)))

    removed.sort(key=bearingSort)
    removed.reverse()

    return 100 * removed[finalIndex].locArr[0] + removed[finalIndex].locArr[1]


class asteroid:
    def __init__(self, locArr):
        self.locArr = locArr[:]
        self.locStr = arrToStr(locArr)
        self.sees = {}
        self.bearing = 0

    def findVisible(self, asteroids):
        others = {}
        for o in asteroids:
            if o != self:
                others[o.locStr] = o

        for other in others:
            if other in self.sees:
                continue

            o = others[other]

            slope = getSlope(self, o)

            point = self.locArr[:]
            for (i, (item1, item2)) in enumerate(zip(point, slope)):
                point[i] = item1 + item2

            visible = True
            while True:
                rounded = [round(x) for x in point]
                if max([abs(item1 - item2) for (item1, item2) in zip(point, rounded)]) < 0.0001:
                    if arrToStr(rounded) == o.locStr:
                        break

                    if arrToStr(rounded) in others:
                        visible = False
                        break

                for (i, (item1, item2)) in enumerate(zip(point, slope)):
                    point[i] = item1 + item2

            if visible:
                self.sees[o.locStr] = o
                o.sees[self.locStr] = self

    def findBearing(self, station):
        self.bearing = math.atan2(self.locArr[0] - station.locArr[0], self.locArr[1] - station.locArr[1])


def getSlope(a1, a2):
    slope = [a2.locArr[0] - a1.locArr[0], a2.locArr[1] - a1.locArr[1]]

    if 1 not in slope:
        if slope[0] == 0:
            slope[1] /= abs(slope[1])
        else:
            div = abs(slope[0])
            for i in range(2):
                slope[i] /= div

    return slope


def arrToStr(arr):
    string = str(arr[0])
    for a in arr[1:]:
        string += ',' + str(a)

    return string


def strToArr(string):
    return [int(x) for x in string.split(',')]


def visibleSort(a):
    return len(a.sees)


def bearingSort(a):
    return a.bearing


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
        print(f"\nPart 1:\nBest station location sees: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\n200th Asteroid Vaporized: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)