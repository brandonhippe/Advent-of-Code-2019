import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
def part1(data, width = 25, height = 6):
    """ 2019 Day 8 Part 1

    >>> part1(['123456789012'], 3, 2)
    1
    """

    lines = [int(x) for x in data[0][::-1]]

    numData = []
    for i in range(len(lines) // (width * height)):
        layerData = [0] * 3
        arr = lines[i * width * height:(i + 1) * width * height]
        for j in range(3):
            layerData[j] = arr.count(j)

        numData.append(layerData)

    numData.sort(key=lambda e: e[0])
    return numData[0][1] * numData[0][2]


def part2(data, width = 25, height = 6):
    """ 2019 Day 8 Part 2
    """

    lines = [int(x) for x in data[0][::-1]]

    visibleImg = [0] * (width * height)
    for layer in range(len(lines) // (width * height)):
        arr = lines[layer * width * height:(layer + 1) * width * height]
        for (i, num) in enumerate(arr):
            if num != 2:
                visibleImg[i] = num

    visibleImg.reverse()
    img = ''
    for y in range(height):
        img += '\n'
        for x in range(width):
            img += '█' if visibleImg[x + y * width] == 1 else ' '

    return img


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
        print(f"\nPart 1:\n1's * 2's in layer with fewest 0's: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nMessage: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)