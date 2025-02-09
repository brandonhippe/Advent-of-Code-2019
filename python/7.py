import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import itertools
from intcode import Intcode


def part1(data):
    """ 2019 Day 7 Part 1

    >>> part1(['3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0'])
    43210
    >>> part1(['3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0'])
    54321
    >>> part1(['3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0'])
    65210
    """

    intcode = Intcode({i: int(x) for i, x in enumerate(data[0].split(','))})
    largest = 0
    orders = list(itertools.permutations(range(5)))
    
    for order in orders:
        output = 0
        for o in order:
            intcode.reset()
            intcode.addInput(o)
            intcode.addInput(output)
            intcode.runCode()
            output = intcode.getOutput()[-1]

        if output > largest:
            largest = output

    return largest


def part2(data):
    """ 2019 Day 7 Part 2

    >>> part2(['3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5'])
    139629729
    >>> part2(['3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10'])
    18216
    """

    intcode = Intcode({i: int(x) for i, x in enumerate(data[0].split(','))})
    orders = list(itertools.permutations(range(5, 10)))
    largest = 0

    for order in orders:
        output = handler(intcode, order)
        if output > largest:
            largest = output

    return largest


def handler(data, ampNums):
    amplifiers = [data.copy() for _ in range(5)]
    for i in range(5):
        amplifiers[i].addInput(ampNums[i])

    pOutput = 0

    while True:
        for i in range(5):
            amplifiers[i].addInput(pOutput)
            amplifiers[i].runCode()

            pOutput = amplifiers[i].getOutput()[0]
            amplifiers[i].resetOutput()

        if sum([a.isDone() for a in amplifiers]) == 5:
            break

    return pOutput


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
        print(f"\nPart 1:\nMaximum Thruster Signal: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nMaximum Thruster Signal: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)