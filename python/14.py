import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import math


def part1(data):
    """ 2019 Day 14 Part 1

    >>> part1(['157 ORE => 5 NZVS', '165 ORE => 6 DCFZ', '44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL', '12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ', '179 ORE => 7 PSHF', '177 ORE => 5 HKGWZ', '7 DCFZ, 7 PSHF => 2 XJWVT', '165 ORE => 2 GPVTF', '3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT'])
    13312
    >>> part1(['2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG', '17 NVRVD, 3 JNWZP => 8 VPVL', '53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL', '22 VJHF, 37 MNCFX => 5 FWMGM', '139 ORE => 4 NVRVD', '144 ORE => 7 JNWZP', '5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC', '5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV', '145 ORE => 6 MNCFX', '1 NVRVD => 8 CXFTF', '1 VJHF, 6 MNCFX => 4 RFSQX', '176 ORE => 6 VJHF'])
    180697
    >>> part1(['171 ORE => 8 CNZTR', '7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL', '114 ORE => 4 BHXH', '14 VRPVC => 6 BMBT', '6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL', '6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT', '15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW', '13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW', '5 BMBT => 4 WPTQ', '189 ORE => 9 KTJDG', '1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP', '12 VRPVC, 27 CNZTR => 2 XDBXC', '15 KTJDG, 12 BHXH => 5 XCVML', '3 BHXH, 2 VRPVC => 7 MZWV', '121 ORE => 7 VRPVC', '7 XCVML => 6 RJRHP', '5 BHXH, 4 VRPVC => 5 LTCX'])
    2210736
    """

    reactions = [reaction(line) for line in data]
    chemicals = {}

    for r in reactions:
        for o in r.outputs:
            chemicals[o] = r

    return oreNeeded(chemicals, 'FUEL', 1)[0]


def part2(data):
    """ 2019 Day 14 Part 2

    >>> part2(['157 ORE => 5 NZVS', '165 ORE => 6 DCFZ', '44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL', '12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ', '179 ORE => 7 PSHF', '177 ORE => 5 HKGWZ', '7 DCFZ, 7 PSHF => 2 XJWVT', '165 ORE => 2 GPVTF', '3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT'])
    82892753
    >>> part2(['2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG', '17 NVRVD, 3 JNWZP => 8 VPVL', '53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL', '22 VJHF, 37 MNCFX => 5 FWMGM', '139 ORE => 4 NVRVD', '144 ORE => 7 JNWZP', '5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC', '5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV', '145 ORE => 6 MNCFX', '1 NVRVD => 8 CXFTF', '1 VJHF, 6 MNCFX => 4 RFSQX', '176 ORE => 6 VJHF'])
    5586022
    >>> part2(['171 ORE => 8 CNZTR', '7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL', '114 ORE => 4 BHXH', '14 VRPVC => 6 BMBT', '6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL', '6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT', '15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW', '13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW', '5 BMBT => 4 WPTQ', '189 ORE => 9 KTJDG', '1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP', '12 VRPVC, 27 CNZTR => 2 XDBXC', '15 KTJDG, 12 BHXH => 5 XCVML', '3 BHXH, 2 VRPVC => 7 MZWV', '121 ORE => 7 VRPVC', '7 XCVML => 6 RJRHP', '5 BHXH, 4 VRPVC => 5 LTCX'])
    460664
    """

    reactions = [reaction(line) for line in data]
    chemicals = {}

    for r in reactions:
        for o in r.outputs:
            chemicals[o] = r

    oneFuel = oreNeeded(chemicals, 'FUEL', 1)[0]

    ore = 1000000000000
    fuelMade = 0

    while True:
        fuelToMake = ore // oneFuel
        
        while True:
            oreUsed, extras = oreNeeded(chemicals, 'FUEL', fuelToMake)

            extrasSum = sum(extras.values())
            if oreUsed >= ore or extrasSum == 0:
                continuing = extrasSum == 0
                break

            fuelToMake += ((ore - oreUsed) // oneFuel) + 1

        if not continuing:
            fuelMade += fuelToMake - 1
            break

        amt = ore // oreUsed
        fuelMade += fuelToMake * amt
        ore -= oreUsed * amt

    return fuelMade


class reaction:
    def __init__(self, line):
        line = line.split(' => ')
        for (i, l) in enumerate(line):
            line[i] = l.split(',')

        self.inputs = {}
        self.outputs = {}

        for inp in line[0]:
            inp = inp.strip()
            inp = inp.split(' ')
            self.inputs[inp[1]] = int(inp[0])

        for out in line[1]:
            out = out.split(' ')
            self.outputs[out[1]] = int(out[0])


def oreNeeded(chemicals, create, amount, extras=None):
    # Calculates # of ore needed to make amount of create using chemicals
    if create == 'ORE':
        return [amount, extras]

    if extras == None:
        extras = {}

    r = chemicals[create]

    reactionNum = math.ceil(amount / r.outputs[create])

    total = 0
    for i in r.inputs:
        needed = r.inputs[i] * reactionNum
        if i in extras:
            needed -= extras[i]
            extras[i] = -needed if needed < 0 else 0

        if needed > 0:
            ore, extras = oreNeeded(chemicals, i, needed, extras)
            total += ore

    if create in extras:
        extras[create] += (r.outputs[create]) * reactionNum - amount
    else:
        extras[create] = (r.outputs[create]) * reactionNum - amount

    return [total, extras]


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
        print(f"\nPart 1:\nAmount of ore needed to make 1 fuel: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nAmount of fuel made with one trillion ore: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)