use std::collections::HashMap;
use std::error::Error;
use csv;
use serde::Deserialize;
use crate::permutation::Permutation;


#[derive(Debug, Clone)]
pub struct Move {
    pub name: String,
    pub permutation: Permutation,
}

#[derive(Debug, Deserialize)]
struct MoveData {
    #[serde(flatten)]
    data: HashMap<String, Vec<usize>>,
}


#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum PuzzleType {
    CUBE(usize),
    WREATH(usize),
    GLOBE(usize, usize),
}

#[derive(Debug)]
pub struct Puzzle {
    pub initial_state: Vec<usize>,
    pub goal_state: Vec<usize>,
    pub moves: Vec<Move>,
    pub num_wildcards: usize,
}

impl PuzzleType {
    pub fn from_str(s: &str) -> Result<PuzzleType, Box<dyn Error>> {
        let parts: Vec<&str> = s.split(['_', '/'].as_ref()).collect();
        match parts[0] {
            "cube" => Ok(PuzzleType::CUBE(parts[1].parse()?)),
            "wreath" => Ok(PuzzleType::WREATH(parts[1].parse()?)),
            "globe" => Ok(PuzzleType::GLOBE(parts[1].parse()?, parts[2].parse()?)),
            _ => Err("Unknown puzzle type".into()),
        }
    }
}

pub fn state_from_str(s: &str) -> Vec<usize> {
    let mut state = Vec::new();
    let mut element_map = HashMap::new();
    let mut next_element = 1;
    for element in s.split(';') {
        if !element_map.contains_key(element) {
            element_map.insert(element, next_element);
            next_element += 1;
        }
        state.push(*element_map.get(element).unwrap());
    }
    state
}

pub fn load_puzzles(puzzle_info_path : &str, puzzles_path: &str) -> Result<Vec<Puzzle>, Box<dyn Error>> {
    let mut puzzle_info_reader = csv::Reader::from_path(puzzle_info_path)?;

    let mut allowed_moves = HashMap::new();
    for record in puzzle_info_reader.records() {
        let record = record?;
        let puzzle_type = PuzzleType::from_str(&record[0])?;

        let mut moves = Vec::new();
        let moves_data: MoveData = serde_json::from_str(&record[1].replace("'", "\"")).expect("Failed to parse moves data");
        for (name, permutation) in moves_data.data {
            moves.push(Move {
                name,
                permutation: Permutation::new(permutation),
            });
        }
        allowed_moves.insert(puzzle_type, moves);
    }

    let mut puzzles : Vec<Puzzle> = Vec::new();
    let mut puzzles_reader = csv::Reader::from_path(puzzles_path)?;
    for record in puzzles_reader.records() {
        let record = record?;
        let puzzle_type = PuzzleType::from_str(&record[1])?;
        let goal_state = state_from_str(&record[2]);
        let initial_state = state_from_str(&record[3]);
        let num_wildcards = record[4].parse()?;
        puzzles.push(Puzzle {
            initial_state,
            goal_state,
            moves: allowed_moves.get(&puzzle_type).unwrap().clone(),
            num_wildcards,
        });
    }
    Ok(puzzles)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_type_from_str() {
        assert_eq!(PuzzleType::from_str("cube_3/3/3").unwrap(), PuzzleType::CUBE(3));
        assert_eq!(PuzzleType::from_str("wreath_3/3/3").unwrap(), PuzzleType::WREATH(3));
        assert_eq!(PuzzleType::from_str("globe_3/4").unwrap(), PuzzleType::GLOBE(3, 4));
        assert!(PuzzleType::from_str("foo").is_err());
    }

    #[test]
    fn test_state_from_str() {
        assert_eq!(state_from_str("a;b;c"), vec![1, 2, 3]);
        assert_eq!(state_from_str("a;b;c;a;b;c"), vec![1, 2, 3, 1, 2, 3]);
        assert_eq!(state_from_str("a;b;c;a;b;c;a;b;c"), vec![1, 2, 3, 1, 2, 3, 1, 2, 3]);
    }

    #[test]
    fn test_load_puzzles() {
        let puzzles = load_puzzles("./../../data/puzzle_info.csv", "./../../data/puzzles.csv").unwrap();
        assert_eq!(puzzles.len(), 398);
        assert_eq!(puzzles[0].initial_state, vec![1, 2, 1, 3, 2, 4, 3, 4, 5, 3, 5, 3, 1, 5, 1, 6, 6, 6, 2, 2, 4, 6, 4, 5]);
        assert_eq!(puzzles[0].goal_state, vec![1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 6,6, 6, 6]);
        assert_eq!(puzzles[0].moves.len(), 6);
    }
}