use log::{debug, info};
use env_logger;

mod permutation;
mod puzzle;

fn main() {
    env_logger::init();

    let args: Vec<String> = std::env::args().collect();
    let puzzle_info_path = if args.len() > 1 { &args[1] } else { "./../../data/puzzle_info.csv" };
    let puzzles_path = if args.len() > 2 { &args[2] } else { "./../../data/puzzles.csv" };
    debug!("Loading puzzle data...");
    let puzzles = puzzle::load_puzzles(puzzle_info_path, puzzles_path).unwrap();
    info!("Loaded {} puzzles", puzzles.len());

}


