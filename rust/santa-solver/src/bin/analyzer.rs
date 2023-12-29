use std::collections::{HashMap, HashSet};
use log::debug;
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
    let solution_path = if args.len() > 3 {
        &args[3]
    } else {
        "./../../python/santa_utils/submission.csv"
    };

    // Load the puzzles
    debug!("Loading puzzle data...");
    let puzzles_info = puzzle::load_puzzle_info(puzzle_info_path).unwrap();
    let puzzles = puzzle::load_puzzles(puzzles_path, &puzzles_info).unwrap();

    // Filter the puzzles: Only keep the puzzles with unique facelets
    let unique_puzzles = puzzles.iter()
        .filter(|p| p.initial_state.iter()
            .collect::<std::collections::HashSet<_>>()
            .len()
            == p.initial_state.len()
        )
        .collect::<Vec<_>>();

    let mut cycles = HashSet::new();
    for p in unique_puzzles {
        // Get the target permutation
        let target_perm = permutation::get_permutation(&p.initial_state, &p.goal_state);
        let perm_info = target_perm.compute_info();
        for c in perm_info.cycles {
            if c.len() > 1 {
                cycles.insert(c);
            }
        }
    }

    // Count the number of elements for each cycle length
    let cycle_orders = cycles.iter()
        .fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c.len()).or_insert(0) += 1;
            acc
        });
    // Print the map for orders >= 2 in a sorted order
    let mut cycle_orders = cycle_orders.iter().collect::<Vec<_>>();
    cycle_orders.sort_by(|a, b| a.0.cmp(b.0));
    for (order, count) in cycle_orders {
        println!("Cycles of order {}: {}", order, count);
    }
}