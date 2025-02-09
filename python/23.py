import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
from intcode import Intcode


def part1(data):
    """ 2019 Day 23 Part 1
    """

    return handlerP1(Intcode({i: int(x) for i, x in enumerate(data[0].split(','))}))


def part2(data):
    """ 2019 Day 23 Part 2
    """

    return handlerP2(Intcode({i: int(x) for i, x in enumerate(data[0].split(','))}))


def handlerP1(intcode: Intcode):
    NICs = []
    inputs = {}
    for i in range(50):
        NICs.append(intcode.copy())
        inputs[i] = [i]

    while True:
        for (i, n) in enumerate(NICs):
            n.addInput(inputs[i])
            inputs[i] = []
            n.runCode()
            outputs = n.getOutput()
            n.resetOutput()

            if len(outputs) >= 3:
                if len(outputs) % 3 != 0:
                    raise ValueError("Invalid output length")
                
                for j in range(0, len(outputs), 3):
                    if outputs[j] == 255:
                        return outputs[j + 2]
                    
                    if len(inputs[outputs[j]]) > 0 and inputs[outputs[j]][0] == -1:
                        inputs[outputs[j]] = outputs[j + 1:j + 3]
                    else:
                        inputs[outputs[j]].extend(outputs[j + 1:j + 3])

        for inp, arr in zip(inputs.keys(), inputs.values()):
            if len(arr) == 0:
                inputs[inp].append(-1)


def handlerP2(intcode: Intcode):
    Nat = []
    NICs = []
    inputs = {}
    for i in range(50):
        NICs.append(intcode.copy())
        inputs[i] = [i]

    lastY = float('inf')
    sleeping = [False] * 50
    lastInput = [0] * 50
    period = [0] * 50
    step = 0
    while True:
        if len(Nat) == 2 and all(sleeping):
            if Nat[-1] == lastY:
                return lastY
            
            lastY = Nat[-1]
            inputs[0] = Nat[:]
            sleeping[0] = False

        nat_len = len(Nat)
        for (i, n) in enumerate(NICs):
            if sleeping[i] and nat_len != 0:
                continue

            emptyInput = inputs[i][-1] == -1
            pLen = len(inputs[i])
            n.addInput(inputs[i])
            inputs[i] = []
            n.runCode()
            outputs = n.getOutput()
            n.resetOutput()

            if emptyInput and pLen != 0 and len(inputs[i]) == 0:
                if step - lastInput[i] == period[i]:
                    sleeping[i] = True
                
                period[i] = step - lastInput[i]
                lastInput[i] = step

            if len(outputs) >= 3:
                if len(outputs) % 3 != 0:
                    raise ValueError("Invalid output length")
                
                for j in range(0, len(outputs), 3):
                    if outputs[j] == 255:
                        Nat = outputs[j + 1:j + 3]
                    elif len(inputs[outputs[j]]) > 0 and inputs[outputs[j]][0] == -1:
                        inputs[outputs[j]] = outputs[j + 1:j + 3]
                        sleeping[outputs[j]] = False
                    else:
                        inputs[outputs[j]].extend(outputs[j + 1:j + 3])

        for inp, arr in zip(inputs.keys(), inputs.values()):
            if len(arr) == 0:
                inputs[inp].append(-1)

        step += 1


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
        print(f"\nPart 1:\nY value of first packet sent to address 255: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nFirst Y value delivered by the NAT to the computer at address 0 twice in a row: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)