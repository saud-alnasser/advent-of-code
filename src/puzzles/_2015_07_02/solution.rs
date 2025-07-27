crate::puzzle!("2015_07_02");

use std::collections::{HashMap, HashSet};

struct Memory {
    vars: HashMap<String, u16>,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<&u16> {
        self.vars.get(key)
    }

    pub fn reset(&mut self) {
        self.vars.clear();
    }

    pub fn apply(&mut self, instructions: &[Instruction]) {
        let mut applied = HashSet::new();

        while applied.len() < instructions.len() {
            for instruction in instructions.iter() {
                if applied.contains(instruction) {
                    continue;
                }

                if self.apply_single(instruction) {
                    applied.insert(instruction);
                }
            }
        }
    }

    fn apply_single(&mut self, instruction: &Instruction) -> bool {
        match instruction {
            Instruction::Set { value, target } => {
                let value = match self.parse_or_get(&value) {
                    Some(value) => value,
                    None => return false,
                };

                self.vars.insert(target.clone(), value);
            }
            Instruction::Not { value, target } => {
                let value = match self.parse_or_get(&value) {
                    Some(value) => value,
                    None => return false,
                };

                self.vars.insert(target.clone(), !value);
            }
            Instruction::And { a, b, target } => {
                let a = match self.parse_or_get(&a) {
                    Some(value) => value,
                    None => return false,
                };

                let b = match self.parse_or_get(&b) {
                    Some(value) => value,
                    None => return false,
                };

                self.vars.insert(target.clone(), a & b);
            }
            Instruction::Or { a, b, target } => {
                let a = match self.parse_or_get(&a) {
                    Some(value) => value,
                    None => return false,
                };

                let b = match self.parse_or_get(&b) {
                    Some(value) => value,
                    None => return false,
                };

                self.vars.insert(target.clone(), a | b);
            }
            Instruction::LShift { a, b, target } => {
                let a = match self.parse_or_get(&a) {
                    Some(value) => value,
                    None => return false,
                };

                let b = match self.parse_or_get(&b) {
                    Some(value) => value,
                    None => return false,
                };

                self.vars.insert(target.clone(), a << b);
            }
            Instruction::RShift { a, b, target } => {
                let a = match self.parse_or_get(&a) {
                    Some(value) => value,
                    None => return false,
                };

                let b = match self.parse_or_get(&b) {
                    Some(value) => value,
                    None => return false,
                };

                self.vars.insert(target.clone(), a >> b);
            }
        }

        true
    }

    fn parse_or_get(&self, value: &str) -> Option<u16> {
        value.parse().ok().or_else(|| self.vars.get(value).copied())
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum Instruction {
    Set {
        value: String,
        target: String,
    },
    Not {
        value: String,
        target: String,
    },
    And {
        a: String,
        b: String,
        target: String,
    },
    Or {
        a: String,
        b: String,
        target: String,
    },
    LShift {
        a: String,
        b: String,
        target: String,
    },
    RShift {
        a: String,
        b: String,
        target: String,
    },
}

impl Solution for Puzzle {
    type Input = Vec<Instruction>;

    fn parse(input: &str) -> Self::Input {
        input
            .lines()
            .map(|line| {
                let parts = line.split(" -> ").collect::<Vec<_>>();
                let lhs = parts[0].split_whitespace().collect::<Vec<_>>();
                let rhs = parts[1];

                match lhs.len() {
                    1 => {
                        let value = lhs[0].to_string();
                        let target = rhs.to_string();

                        Instruction::Set { value, target }
                    }
                    2 => {
                        let value = lhs[1].to_string();
                        let target = rhs.to_string();

                        Instruction::Not { value, target }
                    }
                    _ => {
                        let op = lhs[1];
                        let a = lhs[0].to_string();
                        let b = lhs[2].to_string();
                        let target = rhs.to_string();

                        match op {
                            "AND" => Instruction::And { a, b, target },
                            "OR" => Instruction::Or { a, b, target },
                            "LSHIFT" => Instruction::LShift { a, b, target },
                            "RSHIFT" => Instruction::RShift { a, b, target },
                            _ => unreachable!(),
                        }
                    }
                }
            })
            .collect()
    }

    fn solve(mut instructions: Self::Input) -> Option<String> {
        let mut memory = Memory::new();

        memory.apply(&instructions);

        let a = *memory.get("a").expect("a is not set");

        memory.reset();

        instructions.push(Instruction::Set {
            value: a.to_string(),
            target: "b".to_string(),
        });

        memory.apply(&instructions);

        match memory.get("a") {
            Some(value) => Some(value.to_string()),
            None => None,
        }
    }
}
