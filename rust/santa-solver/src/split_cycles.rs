struct SplitCycles {}

impl SplitCycles {
    /*
     * Splits a cycle at the specified index into two new (not necessarily disjoint) cycles.
     * e.g. (1, 2, 3, 4, 5), split_index = 2 -> (1, 2, 3), (1, 4, 5)
     */
    #[allow(dead_code)]
    pub fn split_cycle(cycle: Vec<usize>, split_index: usize) -> Vec<Vec<usize>> {
        let cycle_len = cycle.len();

        if split_index == 0 || split_index >= cycle_len {
            panic!("Cannot split a cycle at the first or last index");
        }

        let (cycle1, cycle2) = cycle.split_at(split_index + 1);
        let mut result = vec![cycle1.to_vec(), cycle2.to_vec()];
        result[1].push(cycle[0]);

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_split_cycle() {
        let cycle = vec![1, 2, 3, 4, 5];
        let split_index = 2;
        let expected = vec![vec![1, 2, 3], vec![4, 5, 1]];
        let actual = SplitCycles::split_cycle(cycle, split_index);
        assert_eq!(expected, actual);
    }
}
