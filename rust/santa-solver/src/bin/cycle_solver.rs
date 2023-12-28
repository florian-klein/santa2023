use log::debug;
use santa_solver_lib::permutation::Permutation;
use santa_solver_lib::puzzle;
use std::collections::HashMap;
use std::path::Path;

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
    let cycles_path = if args.len() > 2 {
        &args[3]
    } else {
        "./../../data/cycles" // The directory where the generated permutations are stored
    };
    let solution_path = if args.len() > 3 {
        &args[4]
    } else {
        "./../../data/solutions.csv"
    };

    // Load the puzzles
    debug!("Loading puzzle data...");
    let puzzles_info = puzzle::load_puzzle_info(puzzle_info_path).unwrap();
    let puzzles = puzzle::load_puzzles(puzzles_path, &puzzles_info).unwrap();

    // Load the cycles (they are stored in the cycles_path directory)
    // Filenames are of the form puzzle_type_<2/3>c.csv
    debug!("Loading cycles...");
    let mut two_cycles = HashMap::new();
    let mut three_cycles = HashMap::new();

    for (t, _) in puzzles_info.iter() {
        two_cycles.insert(t, HashMap::new());
        three_cycles.insert(t, HashMap::new());
    }

    for (t, _) in puzzles_info.iter() {
        let mut filename = format!("{}/{}_2c.csv", cycles_path, t);
        // Check whether the file exists
        if Path::new(&filename).exists() {
            // Load the 2-cycles (format: permutation, path, length)
            let mut reader = csv::Reader::from_path(filename).unwrap();
            for record in reader.records() {
                let record = record.unwrap();
                let perm: Permutation = record[0].parse().unwrap(); // TODO: Implement FromStr for Permutation
                let path: String = record[1].parse().unwrap();
                two_cycles[t].insert(perm, path);
            }
        }

        filename = format!("{}/{}_3c.csv", cycles_path, t);
        // Check whether the file exists
        if Path::new(&filename).exists() {
            // Load the 3-cycles (format: permutation, path, length)
            let mut reader = csv::Reader::from_path(filename).unwrap();
            for record in reader.records() {
                let record = record.unwrap();
                let perm: Permutation = record[0].parse().unwrap();
                let path: String = record[1].parse().unwrap();
                three_cycles[t].insert(perm, path);
            }
        }
    }

    // Iterate over the puzzles
    for puzzle in puzzles {
        // TODO: solve
    }
}
