import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
from intcode import Intcode


def part1(data):
    """ 2019 Day 9 Part 1

    >>> part1(['109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99'])
    99
    >>> len(str(part1(['1102,34915192,34915192,7,4,7,99,0'])))
    16
    >>> part1(['104,1125899906842624,99'])
    1125899906842624
    """

    intcode = Intcode({i: int(x) for i, x in enumerate(data[0].split(','))})
    intcode.addInput(1)
    if not intcode.runCode():
        raise Exception("Intcode did not halt properly")
    
    return intcode.getOutput()[-1]


def part2(data):
    """ 2019 Day 9 Part 2
    """

    intcode = Intcode({i: int(x) for i, x in enumerate(data[0].split(','))})
    intcode.addInput(2)
    if not intcode.runCode():
        raise Exception("Intcode did not halt properly")
    
    return intcode.getOutput()[-1]


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
        print(f"\nPart 1:\nBOOST Keycode: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nCoordinates of distress signal: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)