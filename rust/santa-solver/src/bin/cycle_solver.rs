use std::collections::HashMap;
use std::path::Path;
use log::{debug, info};
use santa_solver_lib::permutation::Permutation;
use santa_solver_lib::{permutation, puzzle};

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
        "./../../data/cycles"  // The directory where the generated permutations are stored
    };
    let solution_path = if args.len() > 3 {
        &args[4]
    } else {
        "./../../data/solutions"
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
        two_cycles.insert(t, HashMap::<Permutation, String>::new());
        three_cycles.insert(t, HashMap::<Permutation, String>::new());
    }

    for (t, _) in puzzles_info.iter() {
        let mut filename = format!("{}/{}_2c.csv", cycles_path, t);
        // Check whether the file exists
        if Path::new(&filename).exists() {
            // Load the 3-cycles (format: permutation, path, length)
            let mut reader = csv::Reader::from_path(filename).unwrap();
            for record in reader.records() {
                let record = record.unwrap();
                let s : String = record[0].parse().unwrap();
                let p : Vec<usize> = s[1..s.len()-1].split(", ").map(|x| x.parse().unwrap()).collect();
                let perm : Permutation = Permutation::new(p);
                let path : String = record[1].parse().unwrap();
                two_cycles.get_mut(t).unwrap().insert(perm, path);
            }
            info!("Loaded {} 2-cycles for puzzle type {:?}", two_cycles[t].len(), t);
        }
        let filename = format!("{}/{}_3c.csv", cycles_path, t);
        // Check whether the file exists
        if Path::new(&filename).exists() {
            // Load the 3-cycles (format: permutation, path, length)
            let mut reader = csv::Reader::from_path(filename).unwrap();
            for record in reader.records() {
                let record = record.unwrap();
                let s : String = record[0].parse().unwrap();
                let p : Vec<usize> = s[1..s.len()-1].split(", ").map(|x| x.parse().unwrap()).collect();
                let perm : Permutation = Permutation::new(p);
                let path : String = record[1].parse().unwrap();
                three_cycles.get_mut(t).unwrap().insert(perm, path);
            }
            info!("Loaded {} 3-cycles for puzzle type {:?}", three_cycles[t].len(), t);
        }
    }

    // Iterate over the puzzles
    for puzzle in puzzles {
        let mut has_cycles = false;
        // Check whether we have any cycles for this puzzle type
        if let Some(cycles) = two_cycles.get(&puzzle.puzzle_type) {
            if cycles.len() > 0 {
                has_cycles = true;
            }
        }
        if let Some(cycles) = three_cycles.get(&puzzle.puzzle_type) {
            if cycles.len() > 0 {
                has_cycles = true;
            }
        }
        if !has_cycles {
            continue;
        }
        info!("Solving puzzle {} of type {:?} | 2-cycles[{:?}] | 3-cycles[{:?}]", puzzle.id, puzzle.puzzle_type, two_cycles[&puzzle.puzzle_type].len(), three_cycles[&puzzle.puzzle_type].len());
        // Use permutation::decompose to find a solution
        let target = permutation::get_permutation(&puzzle.initial_state, &puzzle.goal_state);
        let mut permutations : Vec<Permutation> = two_cycles[&puzzle.puzzle_type].keys().map(|x| x.clone()).collect();
        permutations.extend(three_cycles[&puzzle.puzzle_type].keys().map(|x| x.clone()));
        let solution = permutation::decompose(&target.compute_info(), &permutations, 30);
        if solution.is_none() {
            debug!("Failed to find a solution for puzzle {} of type {:?}", puzzle.id, puzzle.puzzle_type);
        } else {
            // Build the solution from the paths
            let mut sol = String::new();
            let mut solution_length = 0;
            for (i, p) in solution.unwrap().iter().enumerate() {
                if i > 0 {
                    sol.push('.');
                }
                // Get the path from the two_cycles or three_cycles map
                if let Some(path) = two_cycles[&puzzle.puzzle_type].get(p) {
                    sol.push_str(path);
                    solution_length += path.split('.').count();
                } else if let Some(path) = three_cycles[&puzzle.puzzle_type].get(p) {
                    sol.push_str(path);
                    solution_length += path.split('.').count();
                } else {
                    panic!("Failed to find path for permutation {:?}", p);
                }
            }
            info!("Found solution for puzzle {} of type {:?}: {}", puzzle.id, puzzle.puzzle_type, solution_path);
            // Write the solution to solution_path/<puzzle.id>.csv
            // Create the file if it doesn't exist
            let sol_path = format!("{}/{}.csv", solution_path, puzzle.id);
            if !Path::new(&sol_path).exists() {
                std::fs::File::create(&sol_path).expect("Failed to create file");
            }
            let mut writer = csv::Writer::from_path(sol_path).unwrap();
            writer.write_record(&["id", "moves"]).unwrap();
            writer.write_record(&[&puzzle.id.to_string(), &sol]).unwrap();
        }
    }

}