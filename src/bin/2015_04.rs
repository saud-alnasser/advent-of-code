aoc::puzzle!("2015:04");

use rayon::prelude::*;

impl Solution for Puzzle {
    type Input = String;

    fn parse(input: &str) -> Self::Input {
        input.to_string()
    }

    fn solve_part1(secret: Self::Input) -> Option<String> {
        let max = 10u32.pow(secret.len() as u32);
        let range = 0..max;

        let result = range
            .into_par_iter()
            .find_first(|&i| {
                format!("{:x}", md5::compute(format!("{}{}", secret, i))).starts_with("00000")
            })
            .unwrap();

        Some(result.to_string())
    }

    fn solve_part2(secret: Self::Input) -> Option<String> {
        let max = 10u32.pow(secret.len() as u32);
        let range = 0..max;

        let result = range
            .into_par_iter()
            .find_first(|&i| {
                format!("{:x}", md5::compute(format!("{}{}", secret, i))).starts_with("000000")
            })
            .unwrap();

        Some(result.to_string())
    }
}
