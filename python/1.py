import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
def part1(data):
    """ 2019 Day 1 Part 1

    >>> part1(['12'])
    2
    >>> part1(['14'])
    2
    >>> part1(['1969'])
    654
    >>> part1(['100756'])
    33583
    """

    return sum(int(line) // 3 - 2 for line in data)


def part2(data):
    """ 2019 Day 1 Part 2

    >>> part2(['14'])
    2
    >>> part2(['1969'])
    966
    >>> part2(['100756'])
    50346
    """

    return sum(rocketFuel(int(line)) for line in data)


def rocketFuel(weight):
    newFuel = weight // 3 - 2
    if newFuel <= 0:
        return 0

    newFuel += rocketFuel(newFuel)
    return newFuel


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
        print(f"\nPart 1:\nFuel Required: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nTotal Fuel Required: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)