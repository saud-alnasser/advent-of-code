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

        let name = puzzle.to_string();
        let input = std::fs::read_to_string(puzzle.get_input_path()).expect("unable to read input");

        let (result, time) =
            Runner::timed(|| T::solve_part1(T::parse(&input)).expect("unable to solve part 1"));

        println!("puzzle(1): {} result: {} time: {}", name, result, time);

        let (result, time) =
            Runner::timed(|| T::solve_part2(T::parse(&input)).expect("unable to solve part 2"));

        println!("puzzle(2): {} result: {} time: {}", name, result, time);
    }

    pub fn examples<T: Solution + ToString>(name: T, part: u8) {
        let puzzle = Puzzle::parse(&name.to_string())
            .expect("unable to parse puzzle, expected format: event:day:part");

        let examples: Vec<Example> = serde_json::from_str(
            &std::fs::read_to_string(puzzle.get_examples_path()).expect("unable to read examples"),
        )
        .expect("unable to parse examples");

        for example in examples
            .iter()
            .filter(|e| e.part == part)
            .collect::<Vec<&Example>>()
        {
            let solve = match part {
                1 => T::solve_part1,
                2 => T::solve_part2,
                _ => panic!("invalid part"),
            };

            assert_eq!(
                solve(T::parse(&example.sample))
                    .expect(format!("unable to solve part {}", part).as_str()),
                example.expected
            );
        }
    }

    fn timed<T, F: FnOnce() -> T>(f: F) -> (T, String) {
        let timer = std::time::Instant::now();
        let result = f();
        let elapsed = timer.elapsed();

        let secs = elapsed.as_secs();

        if secs > 0 {
            return (result, format!("{}s", secs));
        }

        let millis = elapsed.as_millis();

        if millis > 0 {
            return (result, format!("{}ms", millis));
        }

        let macros = elapsed.as_micros();

        if macros > 0 {
            return (result, format!("{}µs", macros));
        }

        let nanos = elapsed.as_nanos();

        (result, format!("{}ns", nanos))
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Example {
    pub part: u8,
    pub sample: String,
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
            fn test_puzzle_part1() {
                Runner::examples(Puzzle, 1);
            }

            #[test]
            fn test_puzzle_part2() {
                Runner::examples(Puzzle, 2);
            }
        }
    };
}
