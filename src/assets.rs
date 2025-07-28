use crate::template::Puzzle;

const PUZZLE_BIN_TEMPLATE: &str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/assets/bin.txt"));

const PUZZLE_SOLUTION_TEMPLATE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/assets/solution.txt"
));

const PUZZLE_MOD_TEMPLATE: &str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/assets/mod.txt"));

pub fn bin_template(puzzle: &Puzzle) -> String {
    PUZZLE_BIN_TEMPLATE.replace("%puzzle%", &puzzle.dir_name)
}

pub fn solution_template(puzzle: &Puzzle) -> String {
    PUZZLE_SOLUTION_TEMPLATE.replace("%puzzle%", &puzzle.id)
}

pub fn mod_template(puzzle: &Puzzle) -> String {
    PUZZLE_MOD_TEMPLATE.replace("%puzzle%", &puzzle.id)
}
