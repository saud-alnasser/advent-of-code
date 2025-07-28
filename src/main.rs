use aoc::assets;
use aoc::template::Puzzle;
use aoc_client::AocClient;
use clap::{arg, command, Parser, Subcommand};
use serde_json::json;

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
    if !force {
        if std::path::Path::new(&puzzle.dir_path).exists() {
            eprintln!("puzzle already exists: {}", puzzle.dir_path);
            return;
        }

        if std::path::Path::new(&puzzle.bin_file_path).exists() {
            eprintln!("puzzle already exists: {}", puzzle.bin_file_path);
            return;
        }

        if std::path::Path::new(&puzzle.examples_path).exists() {
            eprintln!("puzzle file already exists: {}", puzzle.examples_path);
            return;
        }
    }

    // create puzzle mod.rs and solution.rs files
    if !std::path::Path::new(&puzzle.dir_path).exists() {
        std::fs::create_dir(&puzzle.dir_path).expect("failed to create puzzle dir");
    }

    std::fs::write(&puzzle.mod_file_path, assets::mod_template(&puzzle))
        .expect("failed to write mod file");

    std::fs::write(&puzzle.solution_path, assets::solution_template(&puzzle))
        .expect("failed to write solution file");

    // ensure puzzles mod file contains the new puzzle

    let mut puzzles_mod_file =
        std::fs::read_to_string("src/puzzles/mod.rs").expect("failed to read puzzles mod");

    if !puzzles_mod_file.contains(&puzzle.dir_name) {
        puzzles_mod_file.push_str(&format!("pub mod {};", puzzle.dir_name));
    }

    std::fs::write("src/puzzles/mod.rs", puzzles_mod_file).expect("failed to write puzzles mod");

    // download puzzle description and input
    let client = AocClient::builder()
        .session_cookie_from_default_locations()
        .expect("failed to get session cookie from default locations on client build")
        .year(puzzle.year.parse().expect("failed to parse year"))
        .expect("failed to set year on client build")
        .day(puzzle.day.parse().expect("failed to parse day"))
        .expect("failed to set day on client build")
        .input_filename(&puzzle.input_path)
        .puzzle_filename(&puzzle.description_path)
        .overwrite_files(true)
        .build()
        .expect("failed to build aoc client");

    client.save_input().expect("failed to save input");
    client
        .save_puzzle_markdown()
        .expect("failed to save puzzle");

    // create examples file (empty)
    std::fs::write(
        &puzzle.examples_path,
        json!(
            [
                {
                    "sample": "1",
                    "expected": "1"
                }
            ]

        )
        .to_string(),
    )
    .expect("failed to write examples file");

    // create puzzle bin_file
    std::fs::write(&puzzle.bin_file_path, assets::bin_template(&puzzle))
        .expect("failed to write bin file");
}

fn solve(puzzle: Puzzle) {
    std::process::Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg(&puzzle.id)
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
        .arg(&puzzle.id)
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .spawn()
        .expect("failed to run cargo")
        .wait()
        .expect("failed to run cargo");
}
