use std::collections::HashMap;
use log::{debug, info};
use env_logger;
use ctrlc2;

mod permutation;
mod puzzle;
mod wreath;
mod kalka_teicher_tsaban;
mod groups;
mod ktt_solver;


fn write_solution_to_file(solution_path: &str, results: &HashMap<usize, String>) {
    debug!("Writing solution to file...");
    let mut writer = csv::Writer::from_path(solution_path).unwrap();
    writer.write_record(&["id", "moves"]).unwrap();
    for (id, moves) in results.iter() {
        writer.write_record(&[&id.to_string(), moves]).unwrap();
    }
    writer.flush().unwrap();
}

fn main() {
    env_logger::init();

    let args: Vec<String> = std::env::args().collect();
    let puzzle_info_path = if args.len() > 1 { &args[1] } else { "./../../data/puzzle_info.csv" };
    let puzzles_path = if args.len() > 2 { &args[2] } else { "./../../data/puzzles.csv" };
    let solution_path = if args.len() > 3 { &args[3] } else { "./../../data/solutions.csv" };
    debug!("Loading puzzle data...");
    let puzzles = puzzle::load_puzzles(puzzle_info_path, puzzles_path).unwrap();
    info!("Loaded {} puzzles", puzzles.len());

    // Catch interrupts so we can write the solution to a file
    ctrlc2::set_handler(move || {
        info!("Caught interrupt, writing solution to file...");
        // TODO: Write solution to file
        std::process::exit(0);
    }).expect("Error setting Ctrl-C handler");

    let wreath_puzzles: Vec<puzzle::Puzzle> = puzzles.iter().filter(|p| {
        if let puzzle::PuzzleType::WREATH(n) = p.puzzle_type {
            n > 12  // We have already solved wreath puzzles of size <=12
        } else {
            false
        }
    }).cloned().collect();
    info!("Solving {} wreath puzzles", wreath_puzzles.len());
    let results = wreath::solve_puzzles(&wreath_puzzles);
    write_solution_to_file(solution_path, &results);
}


