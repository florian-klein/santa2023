use crate::permutation::Permutation;
use crate::permutation::PermutationPath;
use log::{debug, info};
use santa_solver_lib::conjugated_search as search;
use santa_solver_lib::permutation;
use santa_solver_lib::puzzle;
use std::collections::HashMap;

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

    // Load the puzzles
    debug!("Loading puzzle data...");
    let puzzles_info = puzzle::load_puzzle_info(puzzle_info_path).unwrap();
    let puzzles = puzzle::load_puzzles(puzzles_path, &puzzles_info).unwrap();

    for puzzle in puzzles {
        if puzzle
            .initial_state
            .iter()
            .collect::<std::collections::HashSet<_>>()
            .len()
            != puzzle.initial_state.len()
        {
            // continue if we don't have unique elements
            continue;
        }
        info!(
            "Solving puzzle {} of type {:?}",
            puzzle.id, puzzle.puzzle_type,
        );
        let target = permutation::get_permutation(&puzzle.initial_state, &puzzle.goal_state);
        let target_info = target.compute_info();
        let mut target_cycle_lengths: HashMap<usize, Vec<Permutation>> = HashMap::new();
        for cycle in &target_info.cycles {
            let cycle_perm =
                Permutation::from_cycles_fixed_per_size(&vec![cycle.clone()], target.len());
            if target_cycle_lengths.contains_key(&cycle.len()) {
                target_cycle_lengths
                    .get_mut(&cycle.len())
                    .unwrap()
                    .push(cycle_perm);
            } else {
                target_cycle_lengths.insert(cycle.len(), vec![cycle_perm]);
            }
        }
        debug!("We want to reach following target: {:?}", target_info);
        let mut gen_perm_to_index: HashMap<Permutation, usize> = HashMap::new();
        for (i, move_elm) in puzzle.moves.iter().enumerate() {
            gen_perm_to_index.insert(move_elm.permutation.clone(), i);
        }

        // Step 1: Find c-cycles for all cycle lengths in target
        let conjugated_target_group_elements = search::find_c_cycles_relaxed_search(
            &gen_perm_to_index,
            100000,
            target.clone(),
            puzzle.clone(),
        )
        .unwrap();

        // Step 2: Find permutations that we need to build the c-cycles
        for (perm, perm_path) in conjugated_target_group_elements {
            let permutation_target_path = search::find_target_c_cycle_in_conjugated_group(
                target.clone(),
                perm_path,
                perm,
                &puzzle.moves,
            );
            if permutation_target_path.is_some() {
                info!("----------------------------------------");
                info!(
                    "Found target path for this problem! Length: {:?} (todo: verify!!)",
                    permutation_target_path.clone().unwrap().arr.len()
                );
                info!("----------------------------------------");
                let permutation_target_path = permutation_target_path.unwrap();
            }
        }
    }
}
