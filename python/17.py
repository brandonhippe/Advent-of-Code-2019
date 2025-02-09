import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
from intcode import Intcode


def part1(data):
    """ 2019 Day 17 Part 1
    """

    intcode = Intcode({i: int(x) for i, x in enumerate(data[0].split(','))})
    scaffolding = [line for line in handlerP1(intcode).split('\n') if len(line) > 0]

    corners = []
    count = 0
    for (y, line) in enumerate(scaffolding):
        for (x, c) in enumerate(line):
            if c == '#':
                vert = 0
                horiz = 0

                if y + 1 < len(scaffolding) and scaffolding[y + 1][x] != '.':
                    vert += 1

                if y - 1 >= 0 and scaffolding[y - 1][x] != '.':
                    vert += 1

                if x + 1 < len(line) and scaffolding[y][x + 1] != '.':
                    horiz += 1

                if x - 1 >= 0 and scaffolding[y][x - 1] != '.':
                    horiz += 1

                if vert >= 1 and horiz >= 1:
                    corners.append([x, y])

                if vert + horiz >= 3:
                    scaffolding[y] = scaffolding[y][:x] + 'O' + scaffolding[y][x + 1:]
                    count += (x * y)

    return count


def part2(data):
    """ 2019 Day 17 Part 2
    """

    intcode = Intcode({i: int(x) for i, x in enumerate(data[0].split(','))})
    scaffolding = [line for line in handlerP1(intcode).split('\n') if len(line) > 0]

    corners = []
    count = 0
    for (y, line) in enumerate(scaffolding):
        for (x, c) in enumerate(line):
            if c == '#':
                vert = 0
                horiz = 0

                if y + 1 < len(scaffolding) and scaffolding[y + 1][x] != '.':
                    vert += 1

                if y - 1 >= 0 and scaffolding[y - 1][x] != '.':
                    vert += 1

                if x + 1 < len(line) and scaffolding[y][x + 1] != '.':
                    horiz += 1

                if x - 1 >= 0 and scaffolding[y][x - 1] != '.':
                    horiz += 1

                if vert + horiz == 1:
                    end = [x, y]

                if vert >= 1 and horiz >= 1:
                    corners.append([x, y])

                if vert + horiz >= 3:
                    scaffolding[y] = scaffolding[y][:x] + 'O' + scaffolding[y][x + 1:]
                    count += (x * y)

            if c == "^" or c == "v" or c == "<" or c == ">":
                orientation = 0 if c == '^' else (1 if c == '>' else (2 if c == 'v' else 3))
                robot = [x, y]

    corners = orderCorners(scaffolding, robot, end, corners)

    repeats = []

    for (i, c) in enumerate(corners):
        if c in corners[i + 1:]:
            repeats.append([i, i + 1 + corners[i + 1:].index(c)])

    for _ in range(2 ** len(repeats)):
        flip = [x == '1' for x in bin(_)[:1:-1] + '0' * (len(repeats) - len(bin(_)[:1:-1]))]

        path = corners[:]
        for (f, indexes) in zip(flip, repeats):
            if f:
                reversedPart = path[indexes[0] + 1:indexes[1]]
                reversedPart.reverse()
                for (i, c) in enumerate(reversedPart):
                    path[indexes[0] + i + 1] = c

        path.reverse()
        path.append(robot)
        path.reverse()
        path.append(end)

        instructions = generateFunctions(path, orientation)
        if instructions != 0:
            break

    intcode.reset()
    return handlerP2(intcode, instructions[0], instructions[1])


def handlerP1(intcode: Intcode):
    inputs = []

    while True:
        intcode.addInput(inputs)
        if intcode.runCode():
            string = ''
            for c in intcode.getOutput():
                string += chr(c)

            return string
        
        raise Exception("Code requested more input, which could not be provided.")
    

def handlerP2(intcode: Intcode, mainLine, funcs):
    intcode.set_data(0, 2)
    inputs = [ord(c) for c in mainLine + '\n']

    for v in funcs.values():
        for c in v:
            inputs.append(ord(c))

        inputs.append(ord('\n'))

    inputs.append(ord('n'))
    inputs.append(ord('\n'))

    while True:
        intcode.addInput(inputs)
        if intcode.runCode():
            return intcode.getOutput()[-1]
        
        raise Exception("Code requested more input, which could not be provided.")
        

def orderCorners(scaffolding, robot, end, corners):
    orderedCorners = []
    offsets = [[[0, 1], [0, -1]], [[1, 0], [-1, 0]]]

    for offsetIndex in range(len(offsets)):
        pOffset = None
        for o in offsets[offsetIndex]:
            n = [r + c for (r, c) in zip(robot, o)]
            if not (n[0] < 0 or n[0] >= len(scaffolding[0]) or n[1] < 0 or n[1] >= len(scaffolding) or scaffolding[n[1]][n[0]] == '.'):
                pOffset = o
                break

        if pOffset:
            break

    while robot != end:
        n = [r + o for (r, o) in zip(robot, pOffset)]

        if n[0] < 0 or n[0] >= len(scaffolding[0]) or n[1] < 0 or n[1] >= len(scaffolding) or scaffolding[n[1]][n[0]] == '.':
            offsetIndex += 1
            offsetIndex = offsetIndex % 2

            for o in offsets[offsetIndex]:
                n = [r + c for (r, c) in zip(robot, o)]
                if not (n[0] < 0 or n[0] >= len(scaffolding[0]) or n[1] < 0 or n[1] >= len(scaffolding) or scaffolding[n[1]][n[0]] == '.'):
                    pOffset = o
                    break

        robot = n[:]

        if n in corners:
            orderedCorners.append(n)

    return orderedCorners


def functions(mainLine, funcs, func):
    start = 0
    while start < len(mainLine) and ((ord('A') <= ord(mainLine[start]) <= ord('C')) or (mainLine[start] == ',')):
        start += 1

    if func == 'D':
        return [start == len(mainLine) and len(mainLine) <= 20, mainLine]

    for i in range(3, 21):
        finalMain = mainLine[:]
        group = finalMain[start:start + i]
        
        if 'A' in group or 'B' in group or 'C' in group:
            break

        if not (finalMain[start + i] == ',' and (finalMain[start + i + 1] == 'L' or finalMain[start + i + 1] == 'R') or ord('A') <= ord(finalMain[start + i + 1]) <= ord('C')):
            continue

        funcs[func] = group[:]

        finalMain = finalMain.replace(group, func)

        result = functions(finalMain, funcs, chr(ord(func) + 1))

        if result[0]:
            return result

    return [False, mainLine]


def generateFunctions(path, orientation):
    mainLine = ','
    while len(path) > 1:
        pos = path.pop(0)

        if pos[0] == path[0][0]:
            goalOrientation = 2 if path[0][1] > pos[1] else 0
        else:
            goalOrientation = 1 if path[0][0] > pos[0] else 3

        mainLine += 'R,' if (orientation + 1) % 4 == goalOrientation else 'L,'

        orientation = goalOrientation

        while len(path) > 1 and (pos[0] == path[0][0] == path[1][0] or pos[1] == path[0][1] == path[1][1]):
            path.pop(0)

        mainLine += str(sum([abs(l - c) for (l, c) in zip(pos, path[0])])) + ','

    funcs = {'A': '', 'B': '', 'C': ''}
    result = functions(mainLine[1:-1], funcs, 'A')

    if result[0]:
        return [result[1], funcs]
                
    return 0


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
        print(f"\nPart 1:\nSum of alignment parameters: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nDust Collected: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)