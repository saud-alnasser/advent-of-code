use std::fmt::Display;

use aoc_client::{AocClient, SubmissionOutcome};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct Puzzle {
    pub id: String,
    pub year: String,
    pub day: String,
    pub part: String,
    pub bin_file_path: String,
    pub dir_path: String,
    pub dir_name: String,
    pub description_path: String,
    pub input_path: String,
    pub examples_path: String,
    pub mod_file_path: String,
    pub solution_path: String,
}

#[derive(Clone)]
pub struct Metadata {}

impl Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl Puzzle {
    pub fn parse(input: &str) -> Result<Self, String> {
        let parts: Vec<_> = input.split('_').collect();

        if parts.len() != 3 {
            return Err("target must be in the format event:day:part, i.e 2015_01_01".to_string());
        }

        if parts[0].len() != 4 {
            return Err("event must be a 4 digit number".to_string());
        }

        if parts[1].len() > 2 {
            return Err("day must be a 2 digit number".to_string());
        }

        if parts[2].len() > 2 {
            return Err("part must be a 2 digit number".to_string());
        }

        let event_year_n = parts[0]
            .parse::<u16>()
            .map_err(|_| "event must be a number")?;

        let event_day_n = parts[1].parse::<u8>().map_err(|_| "day must be a number")?;

        let puzzle_part_n = parts[2]
            .parse::<u8>()
            .map_err(|_| "part must be a number")?;

        if puzzle_part_n > 2 || puzzle_part_n < 1 {
            return Err("part must be between 1 and 2".to_string());
        }

        if event_day_n > 25 || event_day_n < 1 {
            return Err("day must be between 1 and 25".to_string());
        }

        Ok(Self {
            id: format!(
                "{:04}_{:02}_{:02}",
                event_year_n, event_day_n, puzzle_part_n
            ),
            year: format!("{:04}", event_year_n),
            day: format!("{:02}", event_day_n),
            part: format!("{:02}", puzzle_part_n),
            bin_file_path: format!(
                "src/bin/{:04}_{:02}_{:02}.rs",
                event_year_n, event_day_n, puzzle_part_n
            ),
            dir_path: format!(
                "src/puzzles/_{:04}_{:02}_{:02}",
                event_year_n, event_day_n, puzzle_part_n
            ),
            dir_name: format!(
                "_{:04}_{:02}_{:02}",
                event_year_n, event_day_n, puzzle_part_n
            ),
            description_path: format!(
                "src/puzzles/_{:04}_{:02}_{:02}/description.md",
                event_year_n, event_day_n, puzzle_part_n
            ),
            input_path: format!("data/inputs/{:04}_{:02}.txt", event_year_n, event_day_n),
            examples_path: format!(
                "data/examples/{:04}_{:02}_{:02}.json",
                event_year_n, event_day_n, puzzle_part_n
            ),
            mod_file_path: format!(
                "src/puzzles/_{:04}_{:02}_{:02}/mod.rs",
                event_year_n, event_day_n, puzzle_part_n
            ),
            solution_path: format!(
                "src/puzzles/_{:04}_{:02}_{:02}/solution.rs",
                event_year_n, event_day_n, puzzle_part_n
            ),
        })
    }
}

pub trait Solution {
    type Input;

    fn parse(input: &str) -> Self::Input;

    fn solve(input: Self::Input) -> Option<String>;
}

pub struct Runner;

impl Runner {
    pub fn solve<T: Solution + Display>(name: T) {
        let puzzle = Puzzle::parse(&name.to_string())
            .expect("unable to parse puzzle, expected format: event:day:part");

        let input = std::fs::read_to_string(&puzzle.input_path).expect("unable to read input");
        let parsed = T::parse(&input.trim_end());
        let (output, time) = Runner::timed(|| T::solve(parsed).expect("unable to solve solution"));

        let client = AocClient::builder()
            .session_cookie_from_default_locations()
            .expect("failed to get session cookie from default locations on client build")
            .year(puzzle.year.parse().expect("failed to parse year"))
            .expect("failed to set year on client build")
            .day(puzzle.day.parse().expect("failed to parse day"))
            .expect("failed to set day on client build")
            .build()
            .expect("failed to build aoc client");

        let part = match puzzle.part.as_str() {
            "01" => 1,
            "02" => 2,
            _ => panic!("invalid part"),
        };

        let outcome = client
            .submit_answer(part, &output)
            .expect("failed to submit answer");

        match outcome {
            SubmissionOutcome::Correct => {
                println!(
                    "puzzle[{}]: submit answer={} time={} status=correct",
                    name, output, time
                );
            }
            SubmissionOutcome::Incorrect => {
                println!(
                    "puzzle[{}]: submit answer={} time={} status=incorrect",
                    name, output, time
                );
            }
            SubmissionOutcome::Wait => {
                println!(
                    "puzzle[{}]: submit answer={} time={} status=wait",
                    name, output, time
                );
            }
            SubmissionOutcome::WrongLevel => {
                println!(
                    "puzzle[{}]: submit answer={} time={} status=wrong-level",
                    name, output, time
                );
            }
        }
    }

    pub fn examples<T: Solution + Display>(name: T) {
        let puzzle = Puzzle::parse(&name.to_string())
            .expect("unable to parse puzzle, expected format: event:day:part");

        let examples: Vec<Example> = serde_json::from_str(
            &std::fs::read_to_string(puzzle.examples_path).expect("unable to read examples"),
        )
        .expect("unable to parse examples");

        for example in examples.iter().collect::<Vec<&Example>>() {
            assert_eq!(
                T::solve(T::parse(&example.sample)).expect(format!("unable to solve").as_str()),
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
    pub sample: String,
    pub expected: String,
}

#[macro_export]
macro_rules! puzzle {
    ($name:literal) => {
        use std::fmt::Display;
        use $crate::template::Solution;

        pub struct Puzzle;

        impl Display for Puzzle {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, $name)
            }
        }
    };
}
