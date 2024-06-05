use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct Puzzle {
    event: u16,
    day: u8,
}

impl ToString for Puzzle {
    fn to_string(&self) -> String {
        format!("{:04}:{:02}", self.event, self.day)
    }
}

impl Puzzle {
    pub fn parse(input: &str) -> Result<Self, String> {
        let parts: Vec<_> = input.split(':').collect();

        if parts.len() != 2 {
            return Err("target must be in the format event:day, i.e 2015:01".to_string());
        }

        if parts[0].len() != 4 {
            return Err("event must be a 4 digit number".to_string());
        }

        if parts[1].len() > 2 {
            return Err("day must be a 2 digit number".to_string());
        }

        let event = parts[0].parse().map_err(|_| "event must be a number")?;
        let day = parts[1].parse().map_err(|_| "day must be a number")?;

        if day > 25 || day < 1 {
            return Err("day must be between 1 and 25".to_string());
        }

        Ok(Self { event, day })
    }

    pub fn get_bin_path(&self) -> String {
        format!("src/bin/{:04}_{:02}.rs", self.event, self.day)
    }

    pub fn get_bin_name(&self) -> String {
        format!("{:04}_{:02}", self.event, self.day)
    }

    pub fn get_input_path(&self) -> String {
        format!("data/inputs/{:04}_{:02}.txt", self.event, self.day,)
    }

    pub fn get_examples_path(&self) -> String {
        format!("data/examples/{:04}_{:02}.json", self.event, self.day)
    }
}

pub trait Solution {
    type Input;

    fn parse(input: &str) -> Self::Input;

    fn solve_part1(input: Self::Input) -> Option<String>;

    fn solve_part2(input: Self::Input) -> Option<String>;
}

pub struct Runner;

impl Runner {
    pub fn solve<T: Solution + ToString>(name: T) {
        let puzzle = Puzzle::parse(&name.to_string())
            .expect("unable to parse puzzle, expected format: event:day:part");

        let input = std::fs::read_to_string(puzzle.get_input_path()).expect("unable to read input");

        let part1 = T::solve_part1(T::parse(&input)).expect("unable to solve part 1");
        let part2 = T::solve_part2(T::parse(&input)).expect("unable to solve part 2");

        println!(
            "puzzle: {} result: ({}, {})",
            puzzle.to_string(),
            part1,
            part2
        );
    }

    pub fn examples<T: Solution + ToString>(name: T) {
        let puzzle = Puzzle::parse(&name.to_string())
            .expect("unable to parse puzzle, expected format: event:day:part");

        let examples: Vec<Example> = serde_json::from_str(
            &std::fs::read_to_string(puzzle.get_examples_path()).expect("unable to read examples"),
        )
        .expect("unable to parse examples");

        for example in examples {
            match example.part {
                1 => assert_eq!(
                    T::solve_part1(T::parse(&example.actual)).expect("unable to solve part 1"),
                    example.expected
                ),
                2 => assert_eq!(
                    T::solve_part2(T::parse(&example.actual)).expect("unable to solve part 2"),
                    example.expected
                ),
                _ => panic!("part must be 1 or 2"),
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Example {
    pub part: u8,
    pub actual: String,
    pub expected: String,
}

#[macro_export]
macro_rules! puzzle {
    ($name:literal) => {
        use aoc::template::{Runner, Solution};

        struct Puzzle;

        impl ToString for Puzzle {
            fn to_string(&self) -> String {
                $name.to_string()
            }
        }

        fn main() {
            Runner::solve(Puzzle);
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn test_puzzle() {
                Runner::examples(Puzzle);
            }
        }
    };
}
