use crate::Permutation;
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
}
