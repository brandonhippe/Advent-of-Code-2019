import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
def part1(data, phases = 100):
    """ 2019 Day 16 Part 1

    >>> part1(['12345678'], 4)
    1029498
    >>> part1(['80871224585914546619083218645595'])
    24176176
    >>> part1(['19617804207202209144916044189917'])
    73745418
    >>> part1(['69317163492948606335995924319873'])
    52432133
    """

    nums = [int(x) for x in data[0]]
    for _ in range(phases):
        nums = FFTP1(nums)
    
    return sum([val * 10 ** (7 - i) for (i, val) in enumerate(nums[:8])])


def part2(data):
    """ 2019 Day 16 Part 2

    >>> part2(['03036732577212944063491565474664'])
    84462026
    >>> part2(['02935109699940807407585447034323'])
    78725270
    >>> part2(['03081770884921959731165446850517'])
    53553731
    """

    nums = data[0] * 10000
    offset = int(nums[:7])
    nums = [int(x) for x in nums[offset:]]

    for _ in range(100):
        nums = FFTP2(nums)

    return sum([val * 10 ** (7 - i) for (i, val) in enumerate(nums[:8])])


def FFTP1(data):
    basePattern = [0, 1, 0, -1]
    newData = []
    while len(newData) < len(data):
        val = 0
        for (i, d) in enumerate(data):
            repeat = len(newData) + 1
            index = i + 1
            index = index // repeat
            mult = basePattern[((i + 1) // repeat) % 4]
            val += mult * d
        
        newData.append(abs(val) % 10)

    return newData


def FFTP2(data):
    newData = [0]
    for d in data[::-1]:
        newData.append((d + newData[-1]) % 10)

    return newData[-1:0:-1]


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
        print(f"\nPart 1:\nFirst 8 digits after 100 phases: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nMessage: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)