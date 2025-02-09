use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Intcode {
    mem: HashMap<i64, i64>,
    pc: i64,
    rel_base: i64,
    input: i64,
    outputs: Vec<i64>,
    input_given: bool,
}

impl Intcode {
    pub fn new(mem: Vec<i64>) -> Intcode {
        Intcode {
            mem: mem
                .into_iter()
                .enumerate()
                .map(|(i, x)| (i as i64, x))
                .collect(),
            pc: 0,
            rel_base: 0,
            input: 0,
            outputs: Vec::new(),
            input_given: false,
        }
    }

    pub fn insert(&mut self, k: i64, v: i64) {
        self.mem.insert(k, v);
    }

    pub fn set_input(&mut self, input: i64) {
        self.input = input;
        self.input_given = true;
    }

    pub fn get_output(&self) -> Vec<i64> {
        return self.outputs.clone();
    }

    pub fn clear_output(&mut self) {
        self.outputs.clear();
    }

    pub fn get(&self, k: i64) -> Option<&i64> {
        return self.mem.get(&k);
    }

    pub fn run(&mut self) -> bool {
        loop {
            if !self.mem.contains_key(&self.pc) || self.pc < 0 {
                return false;
            }

            let opcode: i64 = *self.mem.get(&self.pc).unwrap();
            if opcode == 99 {
                return true;
            }

            let (res_reg, op_a, op_b) = self.parse_op(opcode);

            match opcode % 100 {
                1 => {
                    // ADD
                    self.mem.insert(res_reg, op_a.unwrap() + op_b.unwrap());
                    self.pc += 4;
                }
                2 => {
                    // MULT
                    self.mem.insert(res_reg, op_a.unwrap() * op_b.unwrap());
                    self.pc += 4;
                }
                3 => {
                    // INPUT
                    if !self.input_given {
                        return false;
                    }

                    self.input_given = false;
                    self.mem.insert(res_reg, self.input);
                    self.pc += 2;
                }
                4 => {
                    // OUTPUT
                    self.outputs.push(res_reg);
                    self.pc += 2;
                }
                5 => {
                    // JMP IF TRUE
                    self.pc = if op_a.unwrap() != 0 {
                        op_b.unwrap()
                    } else {
                        self.pc + 3
                    };
                }
                6 => {
                    // JMP IF FALSE
                    self.pc = if op_a.unwrap() == 0 {
                        op_b.unwrap()
                    } else {
                        self.pc + 3
                    };
                }
                7 => {
                    // LESS THAN
                    self.mem
                        .insert(res_reg, if op_a.unwrap() < op_b.unwrap() { 1 } else { 0 });
                    self.pc += 4;
                }
                8 => {
                    // EQUALS
                    self.mem
                        .insert(res_reg, if op_a.unwrap() == op_b.unwrap() { 1 } else { 0 });
                    self.pc += 4;
                }
                9 => {
                    // RELATIVE BASE OFFSET
                    self.rel_base += res_reg;
                    self.pc += 2;
                }
                _ => panic!("Opcode {} is invalid", opcode),
            };
        }
    }

    fn parse_op(&self, opcode: i64) -> (i64, Option<i64>, Option<i64>) {
        let op1 = *self.mem.get(&(self.pc + 1)).unwrap();

        let mode_1 = (opcode / 100) % 10;
        let mode_2 = (opcode / 1000) % 10;
        let mode_3 = (opcode / 10000) % 10;

        match opcode % 100 {
            3 => {
                return (
                    match mode_1 {
                        0 => op1,
                        2 => op1 + self.rel_base,
                        _ => panic!("Invalid mode {}", mode_1),
                    },
                    None,
                    None,
                );
            }
            4 | 9 => {
                let res_reg = match mode_1 {
                    0 => self.mem.get(&op1).or(Some(&0)).cloned(),
                    1 => Some(op1),
                    2 => self.mem.get(&(op1 + self.rel_base)).or(Some(&0)).cloned(),
                    _ => panic!("Invalid mode {}", mode_1),
                };

                return (res_reg.unwrap(), None, None);
            }
            _ => {
                let op2 = *self.mem.get(&(self.pc + 2)).or(Some(&0)).unwrap();
                let op3 = *self.mem.get(&(self.pc + 3)).or(Some(&0)).unwrap();

                return (
                    match mode_3 {
                        0 => op3,
                        2 => op3 + self.rel_base,
                        _ => panic!("Invalid mode {}", mode_3),
                    },
                    match mode_1 {
                        0 => self.mem.get(&op1).or(Some(&0)).cloned(),
                        1 => Some(op1),
                        2 => Some(
                            self.mem
                                .get(&(op1 + self.rel_base))
                                .or(Some(&0))
                                .cloned()
                                .unwrap(),
                        ),
                        _ => panic!("Invalid mode {}", mode_1),
                    },
                    match mode_2 {
                        0 => self.mem.get(&op2).or(Some(&0)).cloned(),
                        1 => Some(op2),
                        2 => Some(
                            self.mem
                                .get(&(op2 + self.rel_base))
                                .or(Some(&0))
                                .cloned()
                                .unwrap(),
                        ),
                        _ => panic!("Invalid mode {}", mode_2),
                    },
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}