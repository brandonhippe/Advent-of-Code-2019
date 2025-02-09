import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
from intcode import Intcode


def part1(data):
    """ 2019 Day 13 Part 1
    """

    intcode = Intcode({i: int(x) for i, x in enumerate(data[0].split(','))})
    tiles = handler(intcode)

    count = 0
    for tName in tiles:
        t = tiles[tName]
        if t.id == 2:
            count += 1

    return count


def part2(data):
    """ 2019 Day 13 Part 2
    """

    intcode = Intcode({i: int(x) for i, x in enumerate(data[0].split(','))})
    intcode.set_data(0, 2)
    tiles = handler(intcode)

    for tName in tiles:
        t = tiles[tName]
        
        if t.id == 5:
            break

    return t.score


class tile:
    def __init__(self, info):
        self.x, self.y, self.id = info
        self.string = str(self.x) + ',' + str(self.y)

        if self.x == -1 and self.y == 0:
            self.score = self.id
            self.id = 5


def handler(code, playGame=False):
    inputs = []
    tiles = {}
    while True:
        code.addInput(inputs)
        if code.runCode():
            outputs = code.getOutput()
            for i in range(0, len(outputs), 3):
                newT = tile(outputs[i:i+3])
                tiles[newT.string] = newT

            return tiles
        
        output = code.getOutput()
        code.resetOutput()
        ballX = -1
        paddleX = -1
        for i in range(0, len(output), 3):
            newT = tile(output[i:i+3])
            if newT.id == 3:
                paddleX = newT.x

            if newT.id == 4:
                ballX = newT.x

            tiles[newT.string] = newT

        if playGame:
            printGame(tiles)

        if ballX == paddleX:
            inputs = [0]
        else:
            inputs = [abs(ballX - paddleX) // (ballX - paddleX)]


def printGame(tiles):
    maxs = [float('-inf')] * 2

    for tName in tiles:
        t = tiles[tName]

        if t.x > maxs[0]:
            maxs[0] = t.x

        if t.y > maxs[1]:
            maxs[1] = t.y

    for y in range(maxs[1] + 1):
        for x in range(maxs[0] + 1):
            t = tiles[str(x) + ',' + str(y)]

            c = ' ' if t.id == 0 else ('|' if t.id == 1 and y != 0 else ('-' if t.id == 1 and y == 0 else ('#' if t.id == 2 else ('_' if t.id == 3 else '.'))))
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
        print(f"\nPart 1:\nNumber of block tiles: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nScore after playing game: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)