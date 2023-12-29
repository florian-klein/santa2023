use crate::kalka_teicher_tsaban::factorize;
use crate::permutation::{get_permutation, Permutation, PermutationIndex};
use crate::puzzle::{Puzzle, PuzzleType};
use log::{info, warn};
use std::collections::HashMap;

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
        if puzzle
            .initial_state
            .iter()
            .collect::<std::collections::HashSet<_>>()
            .len()
            != puzzle.initial_state.len()
        {
            continue;
        }
        let target = get_permutation(&puzzle.initial_state, &puzzle.goal_state);
        let target_info = target.compute_info();
        if target_info.signum {
            continue;
        }
        info!(
            "Solving puzzle {} of type {:?}",
            puzzle.id, puzzle.puzzle_type
        );
        // TODO: Move map to puzzle types
        let mut generator_names: HashMap<Permutation, String> = HashMap::new();
        for move_ in puzzle.moves.iter() {
            if generator_names.contains_key(&move_.permutation) {
                warn!(
                    "Duplicate generator for permutation {:?} and {:?}",
                    move_.name,
                    generator_names.get(&move_.permutation).unwrap()
                );
                warn!(
                    "Permutation {:?} will be ignored",
                    move_.permutation.compute_info().cycles
                );
                continue;
            }
            generator_names.insert(move_.permutation.clone(), move_.name.clone());
        }

        let gen_to_str_vec = generator_names
            .into_iter()
            .collect::<Vec<(Permutation, String)>>();
        let gen_to_idx = gen_to_str_vec
            .iter()
            .enumerate()
            .map(|(i, (p, _))| (p.clone(), i))
            .collect::<HashMap<Permutation, PermutationIndex>>();
        let gen_to_str_vec = gen_to_str_vec
            .iter()
            .map(|(_, s)| s.clone())
            .collect::<Vec<String>>();

        let factorization = factorize(&gen_to_idx, gen_to_str_vec, &target);
        if let None = factorization {
            continue;
        }
        let score = factorization.clone().unwrap().split('.').count();
        info!(
            "Solved puzzle {} of type {:?} with score {}",
            puzzle.id, puzzle.puzzle_type, score
        );
        results.insert(puzzle.id, factorization.unwrap());
    }
    results
}
