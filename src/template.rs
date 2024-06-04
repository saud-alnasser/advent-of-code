use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct Puzzle {
    event: u16,
    day: u8,
    part: u8,
}

impl ToString for Puzzle {
    fn to_string(&self) -> String {
        format!("{:04}:{:02}:{:01}", self.event, self.day, self.part)
    }
}

impl Puzzle {
    pub fn parse(input: &str) -> Result<Self, String> {
        let parts: Vec<_> = input.split(':').collect();

        if parts.len() != 3 {
            return Err("target must be in the format event:day:part, i.e 2015:01:1".to_string());
        }

        if parts[0].len() != 4 {
            return Err("event must be a 4 digit number".to_string());
        }

        if parts[1].len() > 2 {
            return Err("day must be a 2 digit number".to_string());
        }

        if parts[2].len() != 1 {
            return Err("part must be a 1 digit number".to_string());
        }

        let event = parts[0].parse().map_err(|_| "event must be a number")?;
        let day = parts[1].parse().map_err(|_| "day must be a number")?;
        let part = parts[2].parse().map_err(|_| "part must be a number")?;

        if day > 25 || day < 1 {
            return Err("day must be between 1 and 25".to_string());
        }

        Ok(Self { event, day, part })
    }

    pub fn get_bin_path(&self) -> String {
        format!(
            "src/bin/{:04}_{:02}_{:01}.rs",
            self.event, self.day, self.part
        )
    }

    pub fn get_bin_name(&self) -> String {
        format!("{:04}_{:02}_{:01}", self.event, self.day, self.part)
    }

    pub fn get_input_path(&self) -> String {
        format!(
            "data/inputs/{:04}_{:02}_{:01}.txt",
            self.event, self.day, self.part
        )
    }

    pub fn get_examples_path(&self) -> String {
        format!(
            "data/examples/{:04}_{:02}_{:01}.json",
            self.event, self.day, self.part
        )
    }
}

pub trait Solution {
    type Input;

    fn parse(input: &str) -> Self::Input;

    fn solve(input: Self::Input) -> Option<String>;
}

pub struct Runner;

impl Runner {
    pub fn solve<T: Solution + ToString>(solution: T) {
        let puzzle = Puzzle::parse(&solution.to_string())
            .expect("unable to parse puzzle, expected format: event:day:part");

        let input = std::fs::read_to_string(puzzle.get_input_path()).expect("unable to read input");

        let input = T::parse(&input);

        let result = T::solve(input).expect("unable to solve puzzle");

        println!("puzzle: {} result: {}", puzzle.to_string(), result);
    }

    pub fn test<T: Solution + ToString>(solution: T) {
        let puzzle = Puzzle::parse(&solution.to_string())
            .expect("unable to parse puzzle, expected format: event:day:part");

        let examples =
            std::fs::read_to_string(puzzle.get_examples_path()).expect("unable to read examples");

        let examples: Vec<Example> =
            serde_json::from_str(&examples).expect("unable to parse examples");

        for example in examples {
            let actual = T::solve(T::parse(&example.actual)).expect("unable to solve example");
            assert_eq!(actual, example.expected);
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Example {
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
                Runner::test(Puzzle);
            }
        }
    };
}
