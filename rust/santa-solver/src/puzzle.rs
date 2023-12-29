use crate::permutation::Permutation;
use csv;
use log::warn;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;

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
pub enum PuzzleType {
    CUBE(usize),
    WREATH(usize),
    GLOBE(usize, usize),
}

#[derive(Debug, Clone)]
pub struct Puzzle {
    pub id: usize,
    pub initial_state: Vec<usize>,
    pub goal_state: Vec<usize>,
    pub moves: Vec<Move>,
    pub num_wildcards: usize,
    pub puzzle_type: PuzzleType,
}

impl PuzzleType {
    pub fn from_str(s: &str) -> Result<PuzzleType, Box<dyn Error>> {
        let parts: Vec<&str> = s.split(['_', '/'].as_ref()).collect();
        match parts[0] {
            "cube" => Ok(PuzzleType::CUBE(parts[1].parse()?)),
            "wreath" => Ok(PuzzleType::WREATH(parts[1].parse()?)),
            "globe" => Ok(PuzzleType::GLOBE(parts[1].parse()?, parts[2].parse()?)),
            _ => Err(format!("Unknown puzzle type {}", parts[0]).into()),
        }
    }
}

pub fn state_from_str(s: &str, element_map: &HashMap<String, usize>) -> Vec<usize> {
    let mut state = Vec::new();

    for element in s.split(';') {
        state.push(
            *element_map
                .get(element)
                .expect(&format!("Unknown element {}", element)),
        );
    }
    state
}

pub fn build_element_map() -> HashMap<String, usize> {
    let mut element_map = HashMap::new();

    // Insert A->1, B->2, C->3, ... , a -> 27, b -> 28, c -> 29, ...
    let mut i = 1;
    for c in "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz".chars() {
        element_map.insert(c.to_string(), i);
        i += 1;
    }

    // Insert N0 -> 1, N1 -> 2, N2 -> 3, ... N1000 -> 100001
    for i in 0..10001 {
        element_map.insert(format!("N{}", i), i + 1);
    }
    element_map
}

pub fn load_puzzle_info(
    puzzle_info_path: &str,
) -> Result<HashMap<PuzzleType, Vec<Move>>, Box<dyn Error>> {
    let mut puzzle_info_reader = csv::Reader::from_path(puzzle_info_path)?;

    let mut allowed_moves = HashMap::new();
    for record in puzzle_info_reader.records() {
        let record = record?;
        let puzzle_type = PuzzleType::from_str(&record[0])?;

        let mut moves = Vec::new();
        let moves_data: MoveData = serde_json::from_str(&record[1].replace("'", "\""))
            .expect("Failed to parse moves data");
        for (name, permutation) in moves_data.data {
            let perm = Permutation::new(permutation.iter().map(|x| *x + 1).collect());
            moves.push(Move {
                name: format!("-{}", name),
                permutation: perm.inverse(),
            });
            moves.push(Move {
                name,
                permutation: perm,
            });
        }
        allowed_moves.insert(puzzle_type, moves);
    }

    Ok(allowed_moves)
}

pub fn load_puzzles(
    puzzles_path: &str,
    allowed_moves: &HashMap<PuzzleType, Vec<Move>>,
) -> Result<Vec<Puzzle>, Box<dyn Error>> {
    let mut puzzles: Vec<Puzzle> = Vec::new();
    let mut puzzles_reader = csv::Reader::from_path(puzzles_path)?;
    let element_map = build_element_map();

    for record in puzzles_reader.records() {
        let record = record?;
        let puzzle_type = PuzzleType::from_str(&record[1])?;
        let goal_state = state_from_str(&record[2], &element_map);
        let initial_state = state_from_str(&record[3], &element_map);
        let num_wildcards = record[4].parse()?;
        puzzles.push(Puzzle {
            id: record[0].parse()?,
            initial_state,
            goal_state,
            moves: allowed_moves.get(&puzzle_type).unwrap().clone(),
            num_wildcards,
            puzzle_type,
        });
    }
    Ok(puzzles)
}

pub fn moves_to_string(moves: &Vec<Move>) -> String {
    let mut s = String::new();
    for (i, m) in moves.iter().enumerate() {
        if i > 0 {
            s.push('.');
        }
        s.push_str(&m.name);
    }
    s
}

pub fn moves_from_string(s: &str, moves: &Vec<Move>) -> Vec<Move> {
    let mut result = Vec::new();
    for name in s.split('.') {
        let mut found = false;
        println!("moves: {:?}", moves);
        println!("name: {}", name);
        for m in moves {
            if m.name == name {
                result.push(m.clone());
                found = true;
                break;
            }
        }
        if !found {
            panic!("Unknown move {}", name);
        }
    }
    result
}

impl Display for PuzzleType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PuzzleType::CUBE(n) => write!(f, "cube_{}_{}_{}", n, n, n),
            PuzzleType::WREATH(n) => write!(f, "wreath_{}_{}", n, n),
            PuzzleType::GLOBE(n, m) => write!(f, "globe_{}_{}", n, m),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_type_from_str() {
        assert_eq!(
            PuzzleType::from_str("cube_3/3/3").unwrap(),
            PuzzleType::CUBE(3)
        );
        assert_eq!(
            PuzzleType::from_str("wreath_3/3/3").unwrap(),
            PuzzleType::WREATH(3)
        );
        assert_eq!(
            PuzzleType::from_str("globe_3/4").unwrap(),
            PuzzleType::GLOBE(3, 4)
        );
        assert!(PuzzleType::from_str("foo").is_err());
    }

    #[test]
    fn test_state_from_str() {
        let element_map = build_element_map();
        assert_eq!(state_from_str("A;B;C", &element_map), vec![1, 2, 3]);
        assert_eq!(state_from_str("B;A;C", &element_map), vec![2, 1, 3]);
        assert_eq!(
            state_from_str("A;B;C;D;E;F", &element_map),
            vec![1, 2, 3, 4, 5, 6]
        );
    }

    #[test]
    fn test_load_puzzles() {
        let puzzle_info = load_puzzle_info("./../../data/puzzle_info.csv").unwrap();
        let puzzles = load_puzzles("./../../data/puzzles.csv", &puzzle_info).unwrap();
        assert_eq!(puzzles.len(), 398);
        assert_eq!(
            puzzles[0].initial_state,
            vec![4, 5, 4, 1, 5, 2, 1, 2, 3, 1, 3, 1, 4, 3, 4, 6, 6, 6, 5, 5, 2, 6, 2, 3]
        );
    }

    #[test]
    fn test_solution() {
        let id = 284;
        let solution = "l.r.l.-r.-l.-r.-r.l.r.-l.r.-l.r.r.-l.-l.r.r.-l.-l.-r.-r.l.r.l.l.l.-r.l.-r.-r.l.-r.-r.-l.-r.l.-r.-r.l.r.-l.-r.-r.l.l.-r.l.-r.-r.l.-r.-r.-l.-r.l.r.r.-l.-r.-l.-r.-r.-l.r.r.l.l.-r.-r.l.l.-r.-r.-r.-l.-r.-r.-l.-r.l.r.-l.r.l.r.r.-l.-l.r.r.-l.-l.-r.-r.l.r.r.l.r.l.-r.-r.-r.-r.-l.-r.-r.-r.-l.-l.r.-l.r.r.-l.-l.-r.-r.l.r.l.l.r.r.l.r.r.-l.-l.-l.-l.-r.-r.-r.l.-r.-r.-l.-r.l.l.r.l.r.l.-r.-r.-l.-l.-r.-r.-r.-l.-l.-l.r.-l.-l.-l.-r.-r.-l.-r.-r.l.-r.-l.-l.-r.-r.-l.-r.l.r.r.r.r.-l.-l.r.l.-r.-l.-r.-r.-l.-r.-r.-r.-l.-l.-l.-r.-r.l.r.-l.-l.-r.l.-r.-l.r.r.r.-l.-l.r.r.-l.-l.-r.-r.-r.-l.-r.-r.-l.-r.l.r.-l.r.l.r.r.-l.-l.-r.-r.l.r.l.r.r.r.-l.-l.r.r.-l.-l.-r.-r.l.r.r.l.r.l.-r.-r.l.l.l.-r.l.-r.-r.l.-r.-r.-l.-r.l.r.-l.-l.-r.-r.l.r.l.r.r.r.r.-l.-l.r.r.-l.-l.-r.-r.l.r.r.l.r.l.-r.-r.-r.l.r.-l.r.l.-r.-r.l.r.r.-l.-l.r.r.-l.-l.-r.-r.l.r.r.l.r.l.-r.-r.-r.l.r.r.l.-r.-l.-r.-r.-r.-r.-l.-l.r.-l.-r.-r.-l";
        let puzzle_info = load_puzzle_info("./../../data/puzzle_info.csv").unwrap();
        let puzzles = load_puzzles("./../../data/puzzles.csv", &puzzle_info).unwrap();
        let puzzle = puzzles.iter().find(|p| p.id == id).unwrap();
        let moves = moves_from_string(solution, &puzzle.moves);
        // Apply the moves to the initial state
        let mut state = puzzle.initial_state.clone();
        println!("Initial state: {:?}", state);
        for m in &moves {
            state = m.permutation.apply(&state);
            println!("Move: {:?} -> {:?}", m.name, state);
        }
        assert_eq!(state, puzzle.goal_state);
    }
}
