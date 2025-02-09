import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
from intcode import Intcode


def part1(data):
    """ 2019 Day 19 Part 1
    """

    return len(handlerP1(Intcode({i: int(x) for i, x in enumerate(data[0].split(','))})))


def part2(data):
    """ 2019 Day 19 Part 2
    """

    corner = handlerP2(Intcode({i: int(x) for i, x in enumerate(data[0].split(','))}))
    return 10000 * corner[0] + corner[1]


def handlerP1(intcode: Intcode):
    affected = []
    y = 0
    prevStart = 0

    while y < 50:
        x = prevStart
        firstFound = False

        while x < 50:
            intcode.reset()
            intcode.addInput([x, y])
            intcode.runCode()
            if intcode.getOutput()[0] == 1:
                if not firstFound:
                    firstFound = True
                    prevStart = x
                affected.append([x, y])
            elif firstFound:
                break

            x += 1

        y += 1

    return affected


def handlerP2(intcode: Intcode):
    y = 99
    prevStart = 0
    while True:
        x = prevStart

        while True:
            intcode.reset()
            intcode.addInput([x, y])
            intcode.runCode()
            if intcode.getOutput()[0] == 1:
                prevStart = x

                intcode.reset()
                intcode.addInput([x + 99, y - 99])
                intcode.runCode()
                if intcode.getOutput()[0] == 1:
                    return [x, y - 99]
                else:
                    break

            x += 1

        y += 1


def printArea(affected):
    for y in range(50):
        for x in range(50):
            c = '.'
            if [x, y] in affected:
                c = '#'
            
            print(c,end='')
        
        print('')


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
        print(f"\nPart 1:\nNumber of points affected by Tractor Beam: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\n10000 * Corner's X + Corner's Y: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)