import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import math, re


def part1(data):
    """ 2019 Day 4 Part 1
    """

    r = [int(x) for x in re.findall(r'\d+', data[0])]

    pw = [(r[0] // 10 ** i) % 10 for i in range(round(math.log10(r[0])))]
    pw.reverse()

    for i in range(len(pw) - 1):
        if pw[i + 1] < pw[i]:
            pw[i + 1] = pw[i]

    found = []
    if len(pw) != len(set(pw)):
        found.append(int(''.join([str(n) for n in pw])))

    while True:
        for i in range(len(pw) - 1, -1, -1):
            if pw[i] != 9:
                pw[i] += 1
                i += 1
                break

        for j in range(i, len(pw)):
            pw[j] = pw[j - 1]

        if len(pw) == len(set(pw)):
            continue

        val = int(''.join([str(n) for n in pw]))
        if val < r[1]:
            found.append(val)
        else:
            break

    return len(found)


def part2(data):
    """ 2019 Day 4 Part 2
    """

    r = [int(x) for x in re.findall(r'\d+', data[0])]

    pw = [(r[0] // 10 ** i) % 10 for i in range(round(math.log10(r[0])))]
    pw.reverse()

    for i in range(len(pw) - 1):
        if pw[i + 1] < pw[i]:
            pw[i + 1] = pw[i]

    found = []

    groups = []
    size = 1
    for i in range(len(pw) - 1):
        if pw[i + 1] != pw[i]:
            groups.append(size)
            size = 1
        else:
            size += 1

    groups.append(size)

    if 2 in groups:
        found.append(int(''.join([str(n) for n in pw])))

    while True:
        for i in range(len(pw) - 1, -1, -1):
            if pw[i] != 9:
                pw[i] += 1
                i += 1
                break

        for j in range(i, len(pw)):
            pw[j] = pw[j - 1]

        val = int(''.join([str(n) for n in pw]))

        if val >= r[1]:
            break

        groups = []
        size = 1
        for i in range(len(pw) - 1):
            if pw[i + 1] != pw[i]:
                groups.append(size)
                size = 1
            else:
                size += 1

        groups.append(size)

        if 2 not in groups:
            continue

        if val < r[1]:
            found.append(val)

    return len(found)


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
        print(f"\nPart 1:\nNumber of valid passwords: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nNumber of valid passwords: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)