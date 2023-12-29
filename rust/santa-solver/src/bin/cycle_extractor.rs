use log::{debug, info, warn};
use santa_solver_lib::kalka_teicher_tsaban as kalka;
use santa_solver_lib::puzzle;
use santa_solver_lib::puzzle::PuzzleType;
use santa_solver_lib::testing_utils::TestingUtils;
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
        10001
    };

    // Load the puzzles
    debug!("Loading puzzle data...");
    let puzzles_info = puzzle::load_puzzle_info(puzzle_info_path).unwrap();

    // Generate transpositions for each puzzle type
    for (puzzle_type, moves) in puzzles_info.iter() {
        // Skip cube puzzles
        if let PuzzleType::CUBE(_) = puzzle_type {
            continue;
        }

        let mut index_to_gen_name = Vec::new();
        let mut gen_to_idx = HashMap::new();
        let mut index_to_gen = Vec::new(); // todo: remove after testin
        let mut counter = 0;

        for m in moves {
            if gen_to_idx.contains_key(&m.permutation) {
                warn!("double elemeent");
                continue;
            }
            gen_to_idx.insert(m.permutation.clone(), counter);
            index_to_gen_name.push(m.name.clone());
            index_to_gen.push(m.permutation.clone());
            counter += 1;
        }
        println!("{:?}", index_to_gen_name);
        let n = moves[0].permutation.len();
        debug!("Generating c-cycles for puzzle type {:?}", puzzle_type);
        let c_cycles = vec![2_usize, 3, 4, 5];
        let mus = kalka::find_c_cycles(&gen_to_idx, &c_cycles, n, max_depth); // TODO: If successful, try to find shorter paths
        if mus.is_none() {
            warn!("Failed to find c-cycles for puzzle type {:?}", puzzle_type);
        } else {
            let mus = mus.unwrap();
            for (c, (mu_path, mu)) in mus.iter() {
                TestingUtils::assert_index_path_equals_permutation(
                    &mu_path.arr,
                    &mu,
                    &index_to_gen,
                );
                debug!("Found {}-cycles for puzzle type {:?}", c, puzzle_type);
                let cycles = kalka::generate_cycles(&gen_to_idx, &mu, &mu_path, 100000);
                // for (perm, path) in &cycles {
                //     TestingUtils::assert_index_path_equals_permutation_using_hashmap(
                //         &path.arr,
                //         &perm,
                //         &gen_to_idx,
                //     );
                // }
                info!(
                    "Generated {} {}-cycles for puzzle type {:?}",
                    cycles.len(),
                    c,
                    puzzle_type
                );

                // Write the transpositions to a new file
                let cycles_file_path = format!("{}/{}_{}c.csv", cycles_path, puzzle_type, c);
                // Create a new file if it doesn't exist
                debug!("Creating file {}", cycles_file_path);
                if !std::path::Path::new(&cycles_file_path).exists() {
                    std::fs::File::create(&cycles_file_path).expect("Failed to create file");
                } else {
                    // TODO: Check for each cycle if it already exists in the file and if the path is shorter
                }
                let mut writer = csv::Writer::from_path(cycles_file_path).unwrap();
                writer
                    .write_record(&["permutation", "path", "length"])
                    .unwrap();
                // Write all cycles to the file
                for (perm, path) in cycles {
                    // TestingUtils::assert_index_path_equals_permutation_using_hashmap(
                    //     &path.arr,
                    //     &perm,
                    //     &gen_to_idx,
                    // );
                    let path_str = path.to_string(&index_to_gen_name);
                    // println!("path {:?}, path_str: {:?}", path, path_str);
                    // println!(
                    //     "index_to_gen_name: {:?}, genidx: {:?}, index to perm {:?}",
                    //     index_to_gen_name, gen_to_idx, index_to_gen
                    // );
                    // TestingUtils::assert_perm_equals_op_string_for_puzzle_type(
                    //     perm.clone(),
                    //     path_str.clone(),
                    //     puzzle_type.clone(),
                    // );
                    let length = path_str.split('.').count();
                    writer
                        .write_record(&[&perm.to_string(), &path_str, &length.to_string()])
                        .unwrap(); // TODO: Find a nicer way to write the permutation
                }
                writer.flush().unwrap();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use santa_solver_lib::{permutation::Permutation, testing_utils::TestingUtils};

    #[test]
    fn test_cycle_manually() {
        let cycle = "f7.-f4.-f4.f9.-f3.f3.-f4.f9.-f3.f3.-f4.f7";
        // Load the moves
        let allowed_moves = puzzle::load_puzzle_info("./../../data/puzzle_info.csv")
            .unwrap()
            .get(&PuzzleType::GLOBE(2, 6))
            .unwrap()
            .clone();
        // Apply the moves to the identity
        let mut perm = Permutation::identity(36);
        for m in cycle.split(".") {
            perm = allowed_moves
                .iter()
                .find(|x| x.name == m)
                .unwrap()
                .permutation
                .compose(&perm);
        }
        println!("Perm: {}", perm);
    }

    #[test]
    fn test_example_csv_move_file() {
        let path = "../../data/cycles/wreath_6_6_2c.csv";
        TestingUtils::validate_cycles_csv(path.to_string(), PuzzleType::WREATH(6));
    }
    // #[test]
    // fn test_example_csv_move_file_three_cycle() {
    //     let path = "../../data/cycles/wreath_7_7_3c.csv";
    //     TestingUtils::validate_cycles_csv(path.to_string(), PuzzleType::WREATH(7));
    // }
}
