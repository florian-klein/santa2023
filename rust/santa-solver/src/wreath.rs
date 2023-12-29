use std::collections::HashMap;
use log::{info};
use crate::permutation::Permutation;
use crate::puzzle::{Move, Puzzle, PuzzleType};


#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum WreathColor {
    A = 1,
    B = 2,
    C = 3,
}

#[derive(Debug)]
struct WreathState {
    state: Vec<WreathColor>
}

impl WreathState {
    pub fn from_puzzle(puzzle: &Puzzle) -> WreathState {
        let mut state = Vec::new();
        for i in 0..puzzle.initial_state.len() {
            match puzzle.initial_state[i] {
                1 => state.push(WreathColor::A),
                2 => state.push(WreathColor::B),
                3 => state.push(WreathColor::C),
                _ => panic!("Invalid color"),
            }
        }
        WreathState {
            state
        }
    }

    fn apply_permutation(&self, permutation: &Permutation) -> Self {
        WreathState {
            state: permutation.apply(&self.state)
        }
    }

    fn wrong_elements(&self, second_c: usize) -> usize {
        let mut count = 0;
        // state[0] and state[second_c] should be C
        // The last |state| / 2 - 1 elements should be B
        for i in 0..(self.state.len() / 2 + 1) {
            if i == 0 || i == second_c {
                if self.state[i] != WreathColor::C {
                    if self.state[i] == WreathColor::A {
                        count += 1;
                    } else {
                        count += 2;
                    }
                }
            } else {
                if self.state[i] != WreathColor::A {
                    if self.state[i] == WreathColor::B {
                        count += 2;
                    } else {
                        count += 1;
                    }
                }
            }
        }
        count
    }
}

fn search(puzzle: &Puzzle, state: &WreathState, second_c: usize, bound: usize, last_move: Option<&str>) -> Option<Vec<Move>> {
    let wrong_elements = state.wrong_elements(second_c);
    let f = wrong_elements / 2;
    if wrong_elements <= puzzle.num_wildcards {
        return Some(Vec::new());
    }
    if f > bound {
        return None;
    }
    let mut min = usize::MAX;
    for mov in &puzzle.moves {
        // Do not apply the inverse of a move that was just applied
        if let Some(last_move) = last_move {
            if mov.name == format!("-{}", last_move) || format!("-{}", mov.name) == last_move {
                continue;
            }
        }
        let new_state = state.apply_permutation(&mov.permutation);
        let result = search(puzzle, &new_state, second_c, bound - 1, Some(&mov.name));
        if result.is_some() {
            let mut result = result.unwrap();
            result.insert(0, mov.clone());
            return Some(result);
        }
        let f = new_state.wrong_elements(second_c) / 2;
        if f < min {
            min = f;
        }
    }
    None
}

fn ida_star(puzzle: &Puzzle) -> Option<Vec<Move>> {
    let PuzzleType::WREATH(n) = puzzle.puzzle_type else { panic!("Invalid puzzle type") };
    let second_c = match n {
        6 | 7 => 2,
        12 => 3,
        21 => 6,
        33 => 9,
        100 => 25,
        _ => panic!("Invalid wreath size"),
    };
    let state = WreathState::from_puzzle(puzzle);
    // Perform IDA* search
    let mut bound = state.wrong_elements(second_c) / 2;
    loop {
        let result = search(puzzle, &state, second_c, bound, None);
        if result.is_some() {
            return result;
        }
        bound += 1;
        if bound > 10000 {
            return None;
        }
    }
}

pub fn solve_puzzles(puzzles: &Vec<Puzzle>) -> HashMap<usize, String> {
    let mut results = HashMap::new();
    for puzzle in puzzles {
        let result = ida_star(puzzle);
        if result.is_some() {
            let result = result.unwrap();
            let score = result.len();
            results.insert(puzzle.id, crate::puzzle::moves_to_string(&result));
            info!("Solved puzzle {:?} of type {:?} with score {}", puzzle.id, puzzle.puzzle_type, score);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::puzzle::{Puzzle, PuzzleType};
    use crate::permutation::Permutation;

    #[test]
    fn test_wreath_state_from_puzzle() {
        let puzzle = Puzzle {
            id: 1,
            initial_state: vec![1, 2, 3, 1, 2, 3, 1, 2, 1, 2],
            goal_state: vec![3, 1, 3, 1, 1, 1, 2, 2, 2, 2],
            moves: vec![
                Move {
                    name: "l".to_string(),
                    permutation: Permutation::new(vec![2, 3, 4, 5, 6, 1, 7, 8, 9, 10]),
                },
                Move {
                    name: "r".to_string(),
                    permutation: Permutation::new(vec![7, 2, 9, 4, 5, 6, 8, 3, 10, 1]),
                },
            ],
            num_wildcards: 0,
            puzzle_type: PuzzleType::WREATH(6),
        };
        let wreath_state = WreathState::from_puzzle(&puzzle);
        assert_eq!(wreath_state.state, vec![WreathColor::A, WreathColor::B, WreathColor::C, WreathColor::A, WreathColor::B, WreathColor::C, WreathColor::A, WreathColor::B, WreathColor::A, WreathColor::B]);
    }

    #[test]
    fn test_wrong_elements() {
        let wreath_state = WreathState {
            state: vec![WreathColor::A, WreathColor::B, WreathColor::C, WreathColor::A, WreathColor::B, WreathColor::C, WreathColor::A, WreathColor::B, WreathColor::A, WreathColor::B],
        };
        assert_eq!(wreath_state.wrong_elements(2), 6);
        let solved_state = WreathState {
            state: vec![WreathColor::C, WreathColor::A, WreathColor::C, WreathColor::A, WreathColor::A, WreathColor::A, WreathColor::B, WreathColor::B, WreathColor::B, WreathColor::B],
        };
        assert_eq!(solved_state.wrong_elements(2), 0);
    }
}