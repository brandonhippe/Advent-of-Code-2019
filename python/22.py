import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
def part1(data):
    """ 2019 Day 22 Part 1
    """

    lines = [line.split(' ') for line in data]

    a, b = [1, 0]
    deckLen = 10007
    
    for line in lines:
        if 'stack' in line:
            c, d = [-1, -1]
        elif 'increment' in line:
            c, d = [int(line[-1]), 0]
        else:
            c, d = [1, -int(line[-1])]

        # Composition of linear shuffles
        a, b = [(a * c) % deckLen, (b * c + d) % deckLen]

    return (2019 * a + b) % deckLen


def part2(data):
    """ 2019 Day 22 Part 2
    """

    lines = [line.split(' ') for line in data]

    a, b = [1, 0]
    deckLen = 119315717514047
    shuffles = 101741582076661
    
    for line in lines:
        if 'stack' in line:
            c, d = [-1, -1]
        elif 'increment' in line:
            c, d = [int(line[-1]), 0]
        else:
            c, d = [1, -int(line[-1])]

        # Composition of linear shuffles
        a, b = [(a * c) % deckLen, (b * c + d) % deckLen]

    # Calculate coeffiecents a, b after applying shuffle sequence for number of shuffles, done by modular exponentiation (repeated composition into itself)
    b = b * (1 - pow(a, shuffles, deckLen)) * modMultInv(1 - a, deckLen)
    a = pow(a, shuffles, deckLen)

    return ((2020 - b) * modMultInv(a, deckLen)) % deckLen


def modMultInv(n, m):
    return pow(n, m - 2, m)


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
        print(f"\nPart 1:\nPosition of card 2019: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nCard at position 2020: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)