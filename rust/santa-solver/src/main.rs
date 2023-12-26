use log::{debug, info};
use env_logger;

mod permutation;
mod puzzle;
mod wreath;

fn main() {
    env_logger::init();

    let args: Vec<String> = std::env::args().collect();
    let puzzle_info_path = if args.len() > 1 { &args[1] } else { "./../../data/puzzle_info.csv" };
    let puzzles_path = if args.len() > 2 { &args[2] } else { "./../../data/puzzles.csv" };
    let solution_path = if args.len() > 3 { &args[3] } else { "./../../data/solutions.csv" };
    debug!("Loading puzzle data...");
    let puzzles = puzzle::load_puzzles(puzzle_info_path, puzzles_path).unwrap();
    info!("Loaded {} puzzles", puzzles.len());
    // Solve all wreath puzzles (Check if puzzle_type is WREATH(_))
    let wreath_puzzles: Vec<puzzle::Puzzle> = puzzles.iter().filter(|p| {
        if let puzzle::PuzzleType::WREATH(n) = p.puzzle_type {
            n > 12  // We have already solved wreath puzzles of size <=12
        } else {
            false
        }
    }).cloned().collect();
    info!("Solving {} wreath puzzles", wreath_puzzles.len());
    let results = wreath::solve_puzzles(&wreath_puzzles);

    debug!("Writing solution to file...");
    // Write the solution to a file
    let mut writer = csv::Writer::from_path(solution_path).unwrap();
    writer.write_record(&["id", "moves"]).unwrap();
    for (id, moves) in results.iter() {
        writer.write_record(&[&id.to_string(), moves]).unwrap();
    }
    writer.flush().unwrap();
}


