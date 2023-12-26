use crate::Permutation;
use std::collections::HashMap;
pub fn parse_permutation_from_cycle(cycle_str: &str, size: usize) -> Permutation {
    let mut result = Permutation::identity(size);

    let cycles: Vec<Vec<usize>> = cycle_str
        .replace(" ", "")
        .trim_matches(|c| c == '(' || c == ')')
        .split(")(")
        .map(|s| s.split(",").map(|n| n.parse::<usize>().unwrap()).collect::<Vec<usize>>())
        .map(|mut cycle| { cycle.push(cycle[0]); cycle })
        .collect();

    for cycle in cycles {
        for i in 0..cycle.len() - 1 {
            result.elements[cycle[i]] = cycle[i + 1];
        }
    }

    result
}

pub fn word_to_perm(word: &str, label_to_gen: HashMap<&str, Permutation>) -> Permutation {
    let mut words = word.split(".");
    let mut result = Permutation::identity(label_to_gen.len());
    
    while let Some(w) = words.next() {
        if let Some(gen) = label_to_gen.get(w) {
            result = result * gen.clone();
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_permutation_from_cycle() {
        let perm1 = parse_permutation_from_cycle("(1,5,7)(2,6,8)", 10);
        let perm2 = parse_permutation_from_cycle("(1,5)(3,4,8,2)", 10);
        assert_eq!(perm1.elements, vec![0, 5, 6, 3, 4, 7, 8, 1, 2, 9]);
        assert_eq!(perm2.elements, vec![0, 5, 3, 4, 8, 1, 6, 7, 2, 9]);
    }

    #[test]
    fn test_permutation_from_cycle() {
        let cycle = "(0,1)(2)";
        let perm1 = parse_permutation_from_cycle(cycle, 3);
        assert_eq!(perm1.elements, vec![1, 0, 2]);
    }

    #[test]
    fn test_word_to_perm() {
        let mut label_to_gen: HashMap<&str, Permutation> = HashMap::new();
        let perm1 = parse_permutation_from_cycle("(0,1)(2)", 3);
        let perm2 = parse_permutation_from_cycle("(0,1)", 3);
        let perm3 = parse_permutation_from_cycle("(0,1,2)", 3);
        label_to_gen.insert("a", perm1.clone());
        label_to_gen.insert("b", perm2.clone());
        label_to_gen.insert("c", perm3.clone());
        let word = "a.b.c";
        let perm = word_to_perm(word, label_to_gen);
        let expected_perm = perm1 * perm2 * perm3;
        assert_eq!(perm.elements, expected_perm.elements);
    }
}
