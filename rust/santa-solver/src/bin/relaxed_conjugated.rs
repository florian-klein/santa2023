use crate::permutation::Permutation;
use log::{debug, info};
use santa_solver_lib::kalka_teicher_tsaban as kalka;
use santa_solver_lib::permutation;
use santa_solver_lib::puzzle;
use std::collections::{HashMap, HashSet};

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
        let mut target_cycle_lengths: HashSet<usize> = HashSet::new();
        for num in &target_info.cycles_id {
            target_cycle_lengths.insert(*num);
        }
        debug!("We want to reach following target: {:?}", target_info);
        let mut gen_perm_to_index: HashMap<Permutation, usize> = HashMap::new();
        for (i, move_elm) in puzzle.moves.iter().enumerate() {
            gen_perm_to_index.insert(move_elm.permutation.clone(), i);
        }
        let result =
            kalka::find_c_cycles_relaxed_search(&gen_perm_to_index, 100, target_cycle_lengths);
        println!("result: {:?}", result);
    }
}
