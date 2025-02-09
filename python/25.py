import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
from intcode import Intcode
from collections import deque, defaultdict
import re


def part1(data):
    """ 2019 Day 25 Part 1
    """

    return handlerP1(Intcode({i: int(x) for i, x in enumerate(data[0].split(','))}))


def part2(data):
    """ 2019 Day 25 Part 2
    """

    return "Christmas has been saved!"


def printASCII(output):
    try:
        for n in output:
            print(chr(n), end='')
    except TypeError:
        print(chr(output), end='')


def handlerP1(intcode: Intcode):
    room_connections = defaultdict(dict)
    stack = deque()

    from_dir = None
    from_room = None

    original_items = {}
    checkpoint = None


    while True:
        intcode.runCode()
        output = ''.join([chr(x) for x in intcode.getOutput()])
        intcode.resetOutput()
        
        curr_room = re.search(r'== (.+) ==', output).group(1)

        door_string = re.search(r'Doors here lead:\n(-\s\w*\n)+\n', output).group(0)
        doors = re.findall(r"\w+\n", door_string)

        item_string = re.search(r'Items here:\n(-\s\w*\n)+\n', output)

        if item_string is not None:
            for item in re.findall(r'\w+\n', item_string.group(0)):
                original_items[item[:-1]] = curr_room

        if from_dir is not None:
            room_connections[from_room][curr_room] = from_dir

        if "ejected back to the checkpoint" in output:
            checkpoint = (from_room, from_dir)
            curr_room = from_room
        elif curr_room not in room_connections:
            changed = True
            while len(doors) != 1 and changed and from_dir is not None:
                changed = False
                if doors[0] == "north\n" and from_dir != "south\n":
                    doors.append(doors.pop(0))
                    changed = True
                elif doors[0] == "south\n" and from_dir != "north\n":
                    doors.append(doors.pop(0))
                    changed = True
                elif doors[0] == "east\n" and from_dir != "west\n":
                    doors.append(doors.pop(0))
                    changed = True
                elif doors[0] == "west\n" and from_dir != "east\n":
                    doors.append(doors.pop(0))
                    changed = True
            
            stack.extend(doors)

        if len(stack) == 0:
            break

        from_dir = stack.pop()
        from_room = curr_room

        intcode.addInput([ord(x) for x in from_dir])

    for intermediate in room_connections.keys():
        for start in room_connections.keys():
            for end in room_connections.keys():
                if intermediate in room_connections[start] and end in room_connections[intermediate]:
                    new_path = f"{room_connections[start][intermediate]}{room_connections[intermediate][end]}"

                    if end in room_connections[start]:
                        if len(new_path.split('\n')) < len(room_connections[start][end].split('\n')):
                            room_connections[start][end] = new_path
                    else:
                        room_connections[start][end] = new_path

    known_dangerous = ["giant electromagnet", "infinite loop"]
    for item in known_dangerous:
        if item in original_items:
            del original_items[item]

    for item, room in original_items.copy().items():
        intcode.reset()
        intcode.addInput([ord(x) for x in room_connections["Hull Breach"][room]])
        intcode.addInput([ord(x) for x in f"take {item}\n"])
        if intcode.runCode():
            del original_items[item]
    
    intcode.reset()
    curr_room = "Hull Breach"
    for item, room in original_items.items():
        intcode.addInput([ord(x) for x in room_connections[curr_room][room]])
        intcode.addInput([ord(x) for x in f"take {item}\n"])

        curr_room = room

    intcode.addInput([ord(x) for x in room_connections[curr_room][checkpoint[0]]])

    for item in original_items.keys():
        intcode.addInput([ord(x) for x in f"drop {item}\n"])

    intcode.runCode()

    item_mapping = {item: 1 << i for i, item in enumerate(original_items.keys())}
    for num in range(1 << len(original_items)):
        intcode.resetOutput()
        take_items = [item for item, mask in item_mapping.items() if num & mask]

        for item in take_items:
            intcode.addInput([ord(x) for x in f"take {item}\n"])

        intcode.addInput([ord(x) for x in checkpoint[1]])

        if intcode.runCode():
            break

        for item in take_items:
            intcode.addInput([ord(x) for x in f"drop {item}\n"])

    return re.findall(r'\d+', ''.join([chr(x) for x in intcode.getOutput()]))[0]


def handlerManual(intcode: Intcode):
    inputs = []

    while True:
        intcode.addInput(inputs)
        if intcode.runCode():
            output = intcode.getOutput()
            printASCII(output)
        else:
            output = intcode.getOutput()
            printASCII(output)
            command = input()
            inputs = [ord(c) for c in command + '\n']


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
        print(f"\nPart 1:\nPassword for the airlock: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\n{p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)