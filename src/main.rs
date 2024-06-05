use aoc::template::Puzzle;
use clap::{arg, command, Parser, Subcommand};
use serde_json::json;

const PUZZLE_TEMPLATE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/assets/puzzle.txt"
));

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CLI {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "scaffolds a new puzzle by creating all necessary files")]
    Scaffold {
        /// puzzle to scaffold it's necessary files
        #[arg(value_parser = Puzzle::parse)]
        puzzle: Puzzle,
        /// force the creation of the files even if they already exist
        #[arg(long)]
        force: bool,
    },
    #[command(about = "runs a puzzle against the input data")]
    Solve {
        // puzzle to run it's solution against the input data
        #[arg(value_parser = Puzzle::parse)]
        puzzle: Puzzle,
    },
    #[command(about = "runs a puzzle against the examples")]
    Examples {
        // puzzle to run it's solution against the examples
        #[arg(value_parser = Puzzle::parse)]
        puzzle: Puzzle,
    },
}

fn main() {
    let cli = CLI::parse();

    match cli.command {
        Some(Commands::Scaffold { puzzle, force }) => scaffold(puzzle, force),
        Some(Commands::Solve { puzzle }) => solve(puzzle),
        Some(Commands::Examples { puzzle }) => examples(puzzle),
        None => {
            eprintln!("no valid command provided");
            std::process::exit(1);
        }
    }
}

fn scaffold(puzzle: Puzzle, force: bool) {
    let bin = puzzle.get_bin_path();
    let input = puzzle.get_input_path();
    let examples = puzzle.get_examples_path();

    if !force {
        if std::path::Path::new(&bin).exists() {
            eprintln!("bin file already exists: {}", bin);
            return;
        }

        if std::path::Path::new(&input).exists() {
            eprintln!("input file already exists: {}", input);
            return;
        }

        if std::path::Path::new(&examples).exists() {
            eprintln!("examples file already exists: {}", examples);
            return;
        }
    }

    std::fs::write(
        bin,
        PUZZLE_TEMPLATE.replace("%puzzle%", puzzle.to_string().as_str()),
    )
    .expect("failed to write bin file");
    std::fs::write(input, "").expect("failed to write input file");
    std::fs::write(
        examples,
        json!(
            [
                {
                    "part": 1,
                    "sample": "1",
                    "expected": "1"
                }
            ]

        )
        .to_string(),
    )
    .expect("failed to write examples file");
}

fn solve(puzzle: Puzzle) {
    std::process::Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg(puzzle.get_bin_name())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .spawn()
        .expect("failed to run cargo")
        .wait()
        .expect("failed to run cargo");
}

fn examples(puzzle: Puzzle) {
    std::process::Command::new("cargo")
        .arg("test")
        .arg("--bin")
        .arg(puzzle.get_bin_name())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .spawn()
        .expect("failed to run cargo")
        .wait()
        .expect("failed to run cargo");
}
