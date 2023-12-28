use std::collections::HashMap;
use log::{info, warn};
use crate::puzzle::{Puzzle, PuzzleType};
use crate::kalka_teicher_tsaban::factorize;
use crate::permutation::{get_permutation, Permutation};

pub fn solve_puzzles(puzzles: &Vec<Puzzle>) -> HashMap<usize, String> {
    let mut results = HashMap::new();
    for puzzle in puzzles {
        // Determine whether the puzzle only has unique elements and no wildcards
        /*if puzzle.num_wildcards > 0 {
            continue;
        } */
        if puzzle.puzzle_type != PuzzleType::GLOBE(3, 4) {
            continue;
        }
        if puzzle.initial_state.iter().collect::<std::collections::HashSet<_>>().len() != puzzle.initial_state.len() {
            continue;
        }
        let target = get_permutation(&puzzle.initial_state, &puzzle.goal_state);
        let target_info = target.compute_info();
        if target_info.signum {
            continue;
        }
        info!("Solving puzzle {} of type {:?}", puzzle.id, puzzle.puzzle_type);
        // TODO: Move map to puzzle types
        let mut generator_names: HashMap<Permutation, String> = HashMap::new();
        for move_ in puzzle.moves.iter() {
            if (generator_names.contains_key(&move_.permutation)) {
                warn!("Duplicate generator for permutation {:?} and {:?}", move_.name, generator_names.get(&move_.permutation).unwrap());
                warn!("Permutation {:?} will be ignored", move_.permutation.compute_info().cycles);
                continue;
            }
            generator_names.insert(move_.permutation.clone(), move_.name.clone());
        }
        let factorization = factorize(&generator_names, &target);
        if let None = factorization {
            continue;
        }
        let score = factorization.clone().unwrap().split('.').count();
        info!("Solved puzzle {} of type {:?} with score {}", puzzle.id, puzzle.puzzle_type, score);
        results.insert(puzzle.id, factorization.unwrap());
    }
    results
}