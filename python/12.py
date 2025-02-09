import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import math


def part1(data, steps = 1000):
    """ 2019 Day 12 Part 1

    >>> part1(['<x=-1, y=0, z=2>', '<x=2, y=-10, z=-7>', '<x=4, y=-8, z=8>', '<x=3, y=5, z=-1>'], 10)
    179
    >>> part1(['<x=-8, y=-10, z=0>', '<x=5, y=5, z=10>', '<x=2, y=-7, z=3>', '<x=9, y=-8, z=-3>'], 100)
    1940
    """

    moons = [moon([int(x.split("=")[-1]) for x in line.strip('<>').split(",")], len(data)) for line in data]

    for _ in range(steps):
        for m in moons:
            m.gravity(moons)

        for m in moons:
            m.timeStep()

    return sum(m.totEng() for m in moons)


def part2(data):
    """ 2019 Day 12 Part 2

    >>> part2(['<x=-1, y=0, z=2>', '<x=2, y=-10, z=-7>', '<x=4, y=-8, z=8>', '<x=3, y=5, z=-1>'])
    2772
    >>> part2(['<x=-8, y=-10, z=0>', '<x=5, y=5, z=10>', '<x=2, y=-7, z=3>', '<x=9, y=-8, z=-3>'])
    4686774924
    """

    moons = [moon([int(x.split("=")[-1]) for x in line.strip('<>').split(",")], len(data)) for line in data]

    step = 0
    cycles = [float('inf')] * 3
    states = []
    for a in range(3):
        temp = {}
        temp[axisState(moons, a)] = step
        states.append(temp)

    while sum(cycles) == float('inf'):
        for m in moons:
            m.gravity(moons)

        for m in moons:
            m.timeStep()

        step += 1

        for a in range(3):
            if cycles[a] == float('inf'):
                state = axisState(moons, a)
                if state in states[a]:
                    cycles[a] = step - states[a][state]
                else:
                    states[a][state] = step

    return lcm(cycles)


class moon:
    def __init__(self, pos, num):
        self.pos = pos[:]
        self.vel = [0] * len(self.pos)
        self.number = num

    def gravity(self, others):
        for other in others:
            if other == self:
                continue

            delta = []
            for (selfC, otherC) in zip(self.pos, other.pos):
                delta.append(selfC - otherC)

            for (i, d) in enumerate(delta):
                self.vel[i] += 1 if d < 0 else (-1 if d > 0 else 0)

    def timeStep(self):
        for (i, v) in enumerate(self.vel):
            self.pos[i] += v

    def potEng(self):
        return sum(abs(x) for x in self.pos)

    def kinEng(self):
        return sum(abs(x) for x in self.vel)

    def totEng(self):
        return self.potEng() * self.kinEng()
    

def axisState(moons, axis):
    string = str(moons[0].pos[axis]) + ',' + str(moons[0].vel[axis])
    for m in moons[1:]:
        string += ',' + str(m.pos[axis]) + ',' + str(m.vel[axis])

    return string


def lcm(arr):
    while len(arr) > 2:
        arr.append(lcm([arr.pop(-1) for _ in range(2)]))

    return abs(arr[0] * arr[1]) // math.gcd(arr[0], arr[1])


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
        print(f"\nPart 1:\nTotal Energy: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nFirst repeated postions will occur at step {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)