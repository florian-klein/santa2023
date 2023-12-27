use std::collections::HashMap;
use log::info;
use crate::puzzle::Puzzle;
use crate::kalka_teicher_tsaban::factorize;

pub fn solve_puzzles(puzzles: &Vec<Puzzle>) -> HashMap<usize, String> {
    let mut results = HashMap::new();
    for puzzle in puzzles {
        // Determine whether the puzzle only has unique elements and no wildcards
        if puzzle.num_wildcards > 0 {
            continue;
        }
        if puzzle.initial_state.unique().len() != puzzle.initial_state.len() {
            continue;
        }
        info!("Solving puzzle {} of type {:?}", puzzle.id, puzzle.puzzle_type);

    }
    results
}