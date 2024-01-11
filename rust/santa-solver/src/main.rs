use ctrlc2;
use env_logger;
use log::{debug, info};
use santa_solver_lib::ktt_solver;
use santa_solver_lib::puzzle;
use std::collections::HashMap;

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
    let puzzle_info_path = if args.len() > 1 {
        &args[1]
    } else {
        "./../../data/puzzle_info.csv"
    };
    let puzzles_path = if args.len() > 2 {
        &args[2]
    } else {
        "./../../data/puzzles.csv"
    };
    let solution_path = if args.len() > 3 {
        &args[3]
    } else {
        "./../../data/solutions.csv"
    };
    debug!("Loading puzzle data...");
    let puzzles_data = puzzle::load_puzzle_info(puzzle_info_path).unwrap();
    let puzzles = puzzle::load_puzzles(puzzles_path, &puzzles_data).unwrap();
    info!("Loaded {} puzzles", puzzles.len());

    // Catch interrupts so we can write the solution to a file
    ctrlc2::set_handler(move || {
        info!("Caught interrupt, writing solution to file...");
        // TODO: Write solution to file
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    let wreath_puzzles: Vec<puzzle::Puzzle> = puzzles
        .iter()
        .filter(|p| {
            if let puzzle::PuzzleType::WREATH(n) = p.puzzle_type {
                n > 12 // We have already solved wreath puzzles of size <=12
            } else {
                false
            }
        })
        .cloned()
        .collect();

    // Count the puzzles with unique elements, no wildcards, and len(initial_state) <= 32
    let mut minko_solve = Vec::new();
    for puzzle in puzzles.iter() {
        if puzzle.num_wildcards > 0 {
            continue;
        }
        if puzzle
            .initial_state
            .iter()
            .collect::<std::collections::HashSet<_>>()
            .len()
            != puzzle.initial_state.len()
        {
            continue;
        }
        if puzzle.initial_state.len() > 32 {
            continue;
        }
        minko_solve.push(puzzle.id);
    }
    println!("Minko solve: {:?}", minko_solve);

    /*
        let wreath_puzzles: Vec<puzzle::Puzzle> = puzzles.iter().filter(|p| {
            if let puzzle::PuzzleType::WREATH(n) = p.puzzle_type {
                n > 12  // We have already solved wreath puzzles of size <=12
            } else {
                false
            }
        }).cloned().collect();
    >>>>>>> 0c62b2c1312d64ba1a7c43b73f916a45beb066d9
        info!("Solving {} wreath puzzles", wreath_puzzles.len());
        let results = wreath::solve_puzzles(&wreath_puzzles); */
    println!("Solving {} wreath puzzles", wreath_puzzles.len());
    let results = ktt_solver::solve_puzzles(&puzzles);
    write_solution_to_file(solution_path, &results);
}
