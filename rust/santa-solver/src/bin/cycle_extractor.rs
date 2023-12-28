use log::{debug, info, warn};
use santa_solver_lib::kalka_teicher_tsaban as kalka;
use santa_solver_lib::puzzle;
use santa_solver_lib::puzzle::PuzzleType;
use std::collections::HashMap;

fn main() {
    env_logger::init();

    let args: Vec<String> = std::env::args().collect();
    let puzzle_info_path = if args.len() > 1 {
        &args[1]
    } else {
        "./../../data/puzzle_info.csv"
    };
    let cycles_path = if args.len() > 2 {
        &args[3]
    } else {
        "./../../data/cycles" // The directory where the generated permutations should be stored
    };

    let max_depth = if args.len() > 3 {
        args[3].parse().unwrap()
    } else {
        12
    };

    // Load the puzzles
    debug!("Loading puzzle data...");
    let puzzles_info = puzzle::load_puzzle_info(puzzle_info_path).unwrap();

    // Generate transpositions for each puzzle type
    for (puzzle_type, moves) in puzzles_info.iter() {
        // Skip Cubes
        if let PuzzleType::CUBE(n) = puzzle_type {
            continue;
        }
        // Skip Globes with n > 19
        if let PuzzleType::GLOBE(n, m) = puzzle_type {
            if *n > 19 || *m > 19 {
                continue;
            }
        }
        let mut gen_to_str = Vec::new();
        let mut gen_to_idx = HashMap::new();

        for m in moves {
            let idx = gen_to_idx.len();
            gen_to_idx.insert(m.permutation.clone(), idx);
            gen_to_str.push((m.name.clone()));
        }
        let n = moves[0].permutation.len();
        info!("Generating transpositions for puzzle type {:?}", puzzle_type);
        let mut mu = kalka::find_c_cycle(&gen_to_idx, 2, n);  // TODO: If successful, try to find shorter paths
        if mu.is_none() {
            warn!("Failed to find 2-cycle for puzzle type {:?}", puzzle_type);
        } else {
            let (mu_path, mu) = mu.unwrap();
            debug!("Found 2-cycle for puzzle type {:?}: {}", puzzle_type, mu_path.to_string(&gen_to_str));

            let transpositions = kalka::generate_transpositions(&gen_to_idx, &mu, &mu_path, 100000);
            debug!("Generated {} transpositions for puzzle type {:?}", transpositions.len(), puzzle_type);

            // Write the transpositions to a new file
            let transpositions_path = format!("{}/{}_2c.csv", cycles_path, puzzle_type);
            // Create a new file if it doesn't exist
            debug!("Creating file {}", transpositions_path);
            if !std::path::Path::new(&transpositions_path).exists() {
                std::fs::File::create(&transpositions_path).expect("Failed to create file");
            } else {
                // TODO: Check for each transposition if it already exists in the file and if the path is shorter
            }
            let mut writer = csv::Writer::from_path(transpositions_path).unwrap();
            writer
                .write_record(&["permutation", "path", "length"])
                .unwrap();
            // Write all transpositions to the file
            for (perm, path) in transpositions {
                let path_str =  path.to_string(&&gen_to_str);
                let length = path_str.split('.').count();
                writer.write_record(&[&perm.to_string(), &path_str, &length.to_string()]).unwrap();  // TODO: Find a nicer way to write the permutation
            }
            writer.flush().unwrap();
        }
        // Try to find a 3-cycle
        mu = kalka::find_c_cycle(&gen_to_idx, 3, n);
        if mu.is_none() {
            warn!("Failed to find 3-cycle for puzzle type {:?}", puzzle_type);
            continue;
        } else {
            let (mu_path, mu) = mu.unwrap();
            debug!("Found 3-cycle for puzzle type {:?}: {}", puzzle_type, mu_path.to_string(&gen_to_str));

            let transpositions = kalka::generate_transpositions(&gen_to_idx, &mu, &mu_path, 100000);
            debug!("Generated {} permutations for puzzle type {:?}", transpositions.len(), puzzle_type);

            // Write the transpositions to a new file
            let transpositions_path = format!("{}/{}_3c.csv", cycles_path, puzzle_type);
            // Create a new file if it doesn't exist
            debug!("Creating file {}", transpositions_path);
            if !std::path::Path::new(&transpositions_path).exists() {
                std::fs::File::create(&transpositions_path).expect("Failed to create file");
            } else {
                // TODO: Check for each transposition if it already exists in the file and if the path is shorter
            }
            let mut writer = csv::Writer::from_path(transpositions_path).unwrap();
            writer
                .write_record(&["permutation", "path", "length"])
                .unwrap();
            // Write all transpositions to the file
            for (perm, path) in transpositions {
                let path_str =  path.to_string(&&gen_to_str);
                let length = path_str.split('.').count();
                writer.write_record(&[&perm.to_string(), &path_str, &length.to_string()]).unwrap();
            }
            writer.flush().unwrap();
        }
    }
}
