from collections import defaultdict

class Intcode:
    def __init__(self, mem: dict) -> None:
        self.default = mem
        self.reset()

    def reset(self) -> None:
        self.data = defaultdict(lambda: 0)
        self.data.update(self.default.copy())
        self.i = 0
        self.relBase = 0
        self.halted = False

        self.inputs = []
        self.outputs = []

    def copy(self) -> 'Intcode':
        new = Intcode(self.default)
        new.data = self.data.copy()
        new.i = self.i
        new.relBase = self.relBase
        new.inputs = self.inputs.copy()
        new.outputs = self.outputs.copy()
        return new

    def runCode(self) -> bool:
        # Modes: 0: Position, 1: Immediate, 2: Relative
        while True:
            line = self.data[self.i] if self.i in self.data else 0

            if line == 99:
                self.halted = True
                # HLT
                break

            opCode = line % 100
            modes = [int(x) for x in str(line // 100)]
            operands = []

            for j in range(self.i + 1, self.i + 4):
                operands.append(self.data[j] if j in self.data else 0)

            if opCode == 1:
                # ADD
                value = 0
                for op in operands[:-1]:
                    mode = modes.pop(-1) if len(modes) != 0 else 0

                    if mode == 0:
                        value += self.data[op] if op in self.data else 0
                    elif mode == 1:
                        value += op
                    elif mode == 2:
                        value += self.data[self.relBase + op] if self.relBase + op in self.data else 0

                mode = modes.pop(-1) if len(modes) != 0 else 0

                if mode == 0:
                    self.data[operands[-1]] = value
                elif mode == 2:
                    self.data[self.relBase + operands[-1]] = value

                self.i += 4
            elif opCode == 2:
                # MULT
                value = 1
                for op in operands[:-1]:
                    mode = modes.pop(-1) if len(modes) != 0 else 0

                    if mode == 0:
                        value *= self.data[op] if op in self.data else 0
                    elif mode == 1:
                        value *= op
                    elif mode == 2:
                        value *= self.data[self.relBase + op] if self.relBase + op in self.data else 0

                mode = modes.pop(-1) if len(modes) != 0 else 0

                if mode == 0:
                    self.data[operands[-1]] = value
                elif mode == 2:
                    self.data[self.relBase + operands[-1]] = value

                self.i += 4
            elif opCode == 3:
                # STR
                mode = modes.pop(-1) if len(modes) != 0 else 0

                if len(self.inputs) == 0:
                    return False
                else:
                    if mode == 0:
                        self.data[operands[0]] = self.inputs.pop(0)
                    elif mode == 2:
                        self.data[self.relBase + operands[0]] = self.inputs.pop(0)

                self.i += 2
            elif opCode == 4:
                # OUT
                mode = modes.pop(-1) if len(modes) != 0 else 0

                if mode == 0:
                    self.outputs.append(self.data[operands[0]] if operands[0] in self.data else 0)
                elif mode == 1:
                    self.outputs.append(operands[0])
                elif mode == 2:
                    self.outputs.append(self.data[self.relBase + operands[0]] if self.relBase + operands[0] in self.data else 0)

                self.i += 2
            elif opCode == 5:
                # JNZ
                mode = modes.pop(-1) if len(modes) != 0 else 0

                if mode == 0:
                    value = self.data[operands[0]] if operands[0] in self.data else 0
                elif mode == 1:
                    value = operands[0]
                elif mode == 2:
                    value = self.data[self.relBase + operands[0]] if self.relBase + operands[0] in self.data else 0

                if value != 0:
                    mode = modes.pop(-1) if len(modes) != 0 else 0

                    if mode == 0:
                        self.i = self.data[operands[1]] if operands[1] in self.data else 0
                    elif mode == 1:
                        self.i = operands[1]
                    elif mode == 2:
                        self.i = self.data[self.relBase + operands[1]] if self.relBase + operands[1] in self.data else 0
                else:
                    self.i += 3
            elif opCode == 6:
                # JZ
                mode = modes.pop(-1) if len(modes) != 0 else 0

                if mode == 0:
                    value = self.data[operands[0]] if operands[0] in self.data else 0
                elif mode == 1:
                    value = operands[0]
                elif mode == 2:
                    value = self.data[self.relBase + operands[0]] if self.relBase + operands[0] in self.data else 0

                if value == 0:
                    mode = modes.pop(-1) if len(modes) != 0 else 0

                    if mode == 0:
                        self.i = self.data[operands[1]] if operands[1] in self.data else 0
                    elif mode == 1:
                        self.i = operands[1]
                    elif mode == 2:
                        self.i = self.data[self.relBase + operands[1]] if self.relBase + operands[1] in self.data else 0
                else:
                    self.i += 3
            elif opCode == 7:
                # LT
                value = 0
                for (j, op) in enumerate(operands[:-1]):
                    mode = modes.pop(-1) if len(modes) != 0 else 0

                    if mode == 0:
                        value += (self.data[op] if op in self.data else 0) * ((-1) ** j)
                    elif mode == 1:
                        value += op * ((-1) ** j)
                    elif mode == 2:
                        value += (self.data[self.relBase + op] if self.relBase + op in self.data else 0) * ((-1) ** j)

                mode = modes.pop(-1) if len(modes) != 0 else 0

                if mode == 0:
                    self.data[operands[-1]] = int(value < 0)
                elif mode == 2:
                    self.data[self.relBase + operands[-1]] = int(value < 0)

                self.i += 4
            elif opCode == 8:
                # EQ
                value = 0
                for (j, op) in enumerate(operands[:-1]):
                    mode = modes.pop(-1) if len(modes) != 0 else 0

                    if mode == 0:
                        value += (self.data[op] if op in self.data else 0) * ((-1) ** j)
                    elif mode == 1:
                        value += op * ((-1) ** j)
                    elif mode == 2:
                        value += (self.data[self.relBase + op] if self.relBase + op in self.data else 0) * ((-1) ** j)

                mode = modes.pop(-1) if len(modes) != 0 else 0

                if mode == 0:
                    self.data[operands[-1]] = int(value == 0)
                elif mode == 2:
                    self.data[self.relBase + operands[-1]] = int(value == 0)

                self.i += 4
            elif opCode == 9:
                # REL BASE
                mode = modes.pop(-1) if len(modes) != 0 else 0

                if mode == 0:
                    self.relBase += self.data[operands[0]] if operands[0] in self.data else 0
                elif mode == 1:
                    self.relBase += operands[0]
                elif mode == 2:
                    self.relBase += self.data[self.relBase + operands[0]] if self.relBase + operands[0] else 0

                self.i += 2

        return True
    
    def addInput(self, value) -> None:
        if isinstance(value, list):
            self.inputs += value
        elif isinstance(value, int):
            self.inputs.append(value)
        else:
            raise ValueError("Invalid input type")
        
    def getOutput(self) -> list:
        return self.outputs
    
    def resetOutput(self) -> None:
        self.outputs = []
        
    def set_data(self, addr, value) -> None:
        self.data[addr] = value

    def get_data(self, addr) -> int:
        if addr in self.data:
            return self.data[addr]
        else:
            raise ValueError("Invalid address")
        
    def isDone(self) -> bool:
        return self.halted