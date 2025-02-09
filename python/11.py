import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
from intcode import Intcode


def part1(data):
    """ 2019 Day 11 Part 1
    """

    intcode = Intcode({i: int(x) for i, x in enumerate(data[0].split(','))})
    return len(handler(intcode, [0]))


def part2(data):
    """ 2019 Day 11 Part 2
    """

    intcode = Intcode({i: int(x) for i, x in enumerate(data[0].split(','))})
    panels = handler(intcode, [1])

    mins = [float('inf')] * 2
    maxs = [float('-inf')] * 2
    for l in panels:
        if panels[l] == 0:
            continue
        
        loc = strToArr(l)

        for (i, c) in enumerate(loc):
            if c < mins[i]:
                mins[i] = c

            if c > maxs[i]:
                maxs[i] = c

    s = ''
    for y in range(maxs[1], mins[1] - 1, -1):
        s += '\n'
        for x in range(mins[0], maxs[0] + 1,):
            c = panels[arrToStr([x, y])] if arrToStr([x, y]) in panels else 0
            s += '█' if c == 1 else ' '

    return s


def handler(code, start):
    inputs = start
    loc = [0, 0]
    lStr = arrToStr(loc)
    facing = [0, 1]
    panels = {}

    while True:
        code.addInput(inputs)
        if code.runCode():
            return panels
        
        outputs = code.getOutput()
        code.resetOutput()

        panels[lStr] = outputs[0]

        if outputs[1] == 0:
            # ROTATE LEFT
            facing = [-facing[1], facing[0]]
        else:
            # ROTATE RIGHT
            facing = [facing[1], -facing[0]]

        for (i, (item1, item2)) in enumerate(zip(loc, facing)):
            loc[i] = item1 + item2

        lStr = arrToStr(loc)
        inputs = panels[lStr] if lStr in panels else 0


def arrToStr(arr):
    string = str(arr[0])
    for a in arr[1:]:
        string += ',' + str(a)
    return string


def strToArr(string):
    return [int(x) for x in string.split(',')]


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
        print(f"\nPart 1:\nNumber of panels painted at least once: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nRegistration Identifier: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)