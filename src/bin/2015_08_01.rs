use aoc::puzzles::_2015_08_01::solution::Puzzle;
use aoc::template::Runner;

fn main() {
    Runner::solve(Puzzle);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        Runner::examples(Puzzle);
    }
}