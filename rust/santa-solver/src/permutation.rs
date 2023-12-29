use crate::groups::DepthLimitedPermutationGroupIterator;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Permutation {
    p: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PermutationInfo<'s> {
    pub permutation: &'s Permutation,
    pub(crate) cycles: Vec<Vec<usize>>,
    pub signum: bool,
}

pub type PermutationIndex = usize;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct PermutationPath {
    pub arr: Vec<PermutationIndex>, // (index, inverse)
}

impl PermutationPath {
    pub fn new(arr: Vec<PermutationIndex>) -> PermutationPath {
        PermutationPath { arr }
    }
    pub fn push(&mut self, index: PermutationIndex) {
        self.arr.push(index);
    }

    pub fn push_multiple(&mut self, indices: &Vec<PermutationIndex>) {
        self.arr.extend(indices);
    }

    pub fn is_empty(&self) -> bool {
        self.arr.is_empty()
    }

    pub fn pow(&mut self, n: usize) {
        let mut result = Vec::new();
        for _ in 0..n {
            result.append(&mut self.arr.clone());
        }
        self.arr = result;
    }

    pub fn merge(&mut self, other: &PermutationPath) {
        self.arr.extend(other.arr.clone());
    }

    pub fn to_string(&self, gen_to_str: &Vec<String>) -> String {
        let result = self
            .arr
            .clone()
            .into_iter()
            .map(|i| gen_to_str[i].clone())
            .collect::<Vec<_>>()
            .join(".");
        result
    }
}

#[derive(Debug, Clone)]
pub struct CompressedPermutation {
    m: HashMap<usize, usize>,
}

impl Permutation {
    pub fn new(p: Vec<usize>) -> Permutation {
        Permutation { p }
    }

    pub fn from_cycles(cycles: &Vec<Vec<usize>>) -> Permutation {
        // warn: does not work when not all elements are visible in cycle (e.g [1,2] instead of
        // [1,2][3]
        let mut p = vec![0; cycles.iter().map(|c| c.len()).sum()];
        for cycle in cycles {
            for i in 0..cycle.len() {
                p[cycle[i] - 1] = cycle[(i + 1) % cycle.len()];
            }
        }
        Permutation::new(p)
    }

    pub fn from_cycles_fixed_per_size(cycles: &Vec<Vec<usize>>, perm_size: usize) -> Permutation {
        let mut p = (1..=perm_size).collect::<Vec<usize>>();
        for cycle in cycles {
            for i in 0..cycle.len() {
                p[cycle[i] - 1] = cycle[(i + 1) % cycle.len()];
            }
        }
        Permutation::new(p)
    }

    pub fn identity(n: usize) -> Permutation {
        let mut p = vec![0; n];
        for i in 0..n {
            p[i] = i + 1;
        }
        Permutation::new(p)
    }

    pub fn len(&self) -> usize {
        self.p.len()
    }

    pub fn get_vec(&self) -> &Vec<usize> {
        &self.p
    }

    pub fn apply<T: Clone>(&self, v: &Vec<T>) -> Vec<T> {
        let mut result = vec![v[0].clone(); self.len()];
        for i in 0..self.len() {
            result[i] = v[self.p[i] - 1].clone();
        }
        result
    }

    pub fn compose_in_place(&mut self, other: &Permutation) {
        todo!();
    }

    pub fn compose(&self, other: &Permutation) -> Permutation {
        let mut result = vec![0; self.len()];
        for i in 0..self.len() {
            result[i] = self.p[other.p[i] - 1];
        }
        Permutation::new(result)
    }

    pub fn inverse(&self) -> Permutation {
        let mut result = vec![0; self.len()];
        for i in 0..self.len() {
            result[self.p[i] - 1] = i + 1;
        }
        Permutation::new(result)
    }

    pub fn pow(&self, n: usize) -> Permutation {
        if n == 0 {
            return Permutation::identity(self.len());
        }
        let mut res = self.p.clone();
        for _ in 0..(n - 1) {
            for i in 0..self.len() {
                res[i] = self.p[res[i] - 1];
            }
        }
        Permutation::new(res)
    }

    pub fn compress(&self) -> CompressedPermutation {
        let mut m = HashMap::new();
        for i in 0..self.len() {
            if self.p[i] != i + 1 {
                m.insert(i + 1, self.p[i]);
            }
        }
        CompressedPermutation::new(m)
    }

    //early exit, if a cycle is longer than max
    pub fn cycle_decomposition_max(&self, max: usize) -> Option<Vec<Vec<usize>>> {
        let mut cycles = Vec::new();
        let mut visited = vec![false; self.len()];
        for i in 0..self.len() {
            if !visited[i] {
                let mut cycle = Vec::new();
                let mut cur_cycle_len = 0;
                let mut j = i;
                while !visited[j] {
                    visited[j] = true;
                    cycle.push(j + 1);
                    cur_cycle_len += 1;
                    if cur_cycle_len > max {
                        return None;
                    }
                    j = self.p[j] - 1;
                }
                cycles.push(cycle);
            }
        }
        Some(cycles)
    }

    pub fn compute_info(&self) -> PermutationInfo {
        let mut cycles = Vec::with_capacity(64);
        let cycle_len = self.len() / 2;
        let mut visited = vec![false; self.len()];
        let mut even_cycles = 0;
        for i in 0..self.len() {
            if !visited[i] {
                let mut cycle = Vec::with_capacity(cycle_len);
                let mut j = i;
                while !visited[j] {
                    visited[j] = true;
                    cycle.push(j + 1);
                    j = self.p[j] - 1;
                }
                if cycle.len() % 2 == 0 {
                    even_cycles += 1;
                }
                cycles.push(cycle);
            }
        }
        // The permutation is even iff the number of even cycles is even
        let signum = even_cycles % 2 == 0;
        PermutationInfo {
            permutation: self,
            cycles,
            signum,
        }
    }

    pub fn parse_permutation_from_cycle(cycle_str: &str, size: usize) -> Permutation {
        let mut elements = (1..=size).collect::<Vec<usize>>();

        let cycles: Vec<Vec<usize>> = cycle_str
            .replace(" ", "")
            .trim_matches(|c| c == '(' || c == ')')
            .split(")(")
            .map(|s| {
                s.split(",")
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>()
            })
            .map(|mut cycle| {
                cycle.push(cycle[0]);
                cycle
            })
            .collect();

        for cycle in cycles {
            for i in 0..(cycle.len() - 1) {
                elements[cycle[i] - 1] = cycle[i + 1];
            }
        }

        Permutation::new(elements)
    }
}

impl Display for Permutation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.p)
    }
}

impl Display for PermutationInfo<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.cycles)?;
        if self.signum {
            write!(f, " (even)")?;
        } else {
            write!(f, " (odd)")?;
        }
        Ok(())
    }
}

impl CompressedPermutation {
    pub fn new(m: HashMap<usize, usize>) -> CompressedPermutation {
        CompressedPermutation { m }
    }

    pub fn identity() -> CompressedPermutation {
        let m = HashMap::new();
        CompressedPermutation::new(m)
    }

    pub fn get(&self, i: usize) -> usize {
        match self.m.get(&i) {
            Some(j) => *j,
            None => i,
        }
    }

    pub fn compose(&self, other: &CompressedPermutation) -> CompressedPermutation {
        let mut m = HashMap::new();
        for (i, j) in other.m.iter() {
            m.insert(*i, self.get(*j));
        }
        for (i, j) in self.m.iter() {
            if !m.contains_key(i) {
                m.insert(*i, *j);
            }
        }
        CompressedPermutation::new(m)
    }

    pub fn inverse(&self) -> CompressedPermutation {
        let mut m = HashMap::new();
        for (i, j) in self.m.iter() {
            m.insert(*j, *i);
        }
        CompressedPermutation::new(m)
    }
}

impl Display for CompressedPermutation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut keys: Vec<&usize> = self.m.keys().collect();
        keys.sort();
        write!(f, "{{")?;
        for key in keys {
            write!(f, "{}: {}, ", key, self.m[key])?;
        }
        write!(f, "}}")
    }
}

pub fn get_permutation<T: PartialEq>(source: &Vec<T>, target: &Vec<T>) -> Permutation {
    let mut p = vec![0; source.len()];
    for i in 0..source.len() {
        p[i] = target.iter().position(|x| x == &source[i]).unwrap() + 1;
    }
    Permutation::new(p)
}

pub fn decompose(
    p: &PermutationInfo,
    t: &Vec<Permutation>,
    depth: usize,
) -> Option<Vec<Permutation>> {
    // Attempts to write p as a product of permutations in t
    // Perform a BFS on the Cayley graph of the group generated by t
    // Decompose every cycle of p individually  // TODO: Check whether this is necessary
    let mut result = Vec::new();
    'outer: for cycle in &p.cycles {
        if cycle.len() == 1 {
            continue;
        }
        let generator = DepthLimitedPermutationGroupIterator::new(t, depth);
        for (p, path) in generator {
            // TODO: improve efficiency: idea: cycle is pretty small, so we just check
            if p == Permutation::from_cycles_fixed_per_size(&vec![cycle.clone()], p.len()) {
                // Found a decomposition
                for i in path {
                    result.push(t[i].clone());
                }
                continue 'outer;
            }
        }
        return None;
    }
    Some(result)
}

#[cfg(test)]
mod permutation_tests {
    use super::*;

    #[test]
    fn test_new() {
        let p = Permutation::new(vec![1, 2, 3]);
        assert_eq!(p.p, vec![1, 2, 3]);
    }

    #[test]
    fn test_from_cycles() {
        let cycles = vec![vec![1, 2, 3], vec![4, 5]];
        let p = Permutation::from_cycles(&cycles);
        assert_eq!(p.p, vec![2, 3, 1, 5, 4]);
    }

    #[test]
    fn test_from_cycles_with_specified_size() {
        let cycles = vec![vec![2, 3]];
        let p = Permutation::from_cycles_fixed_per_size(&cycles, 5);
        assert_eq!(p.p, vec![1, 3, 2, 4, 5]);
    }

    #[test]
    fn test_display() {
        let p = Permutation::new(vec![1, 2, 3]);
        assert_eq!(format!("{}", p), "[1, 2, 3]");
    }

    #[test]
    fn test_len() {
        let p = Permutation::new(vec![1, 2, 3]);
        assert_eq!(p.len(), 3);
    }

    #[test]
    fn test_get_vec() {
        let p = Permutation::new(vec![1, 2, 3]);
        assert_eq!(p.get_vec(), &vec![1, 2, 3]);
    }

    #[test]
    fn test_identity() {
        let p = Permutation::identity(3);
        assert_eq!(p.p, vec![1, 2, 3]);
    }

    #[test]
    fn test_compose() {
        let p = Permutation::new(vec![1, 2, 3]);
        let q = Permutation::new(vec![2, 3, 1]);
        let r = p.compose(&q);
        assert_eq!(r.p, vec![2, 3, 1]);
    }

    #[test]
    fn test_apply() {
        let p = Permutation::new(vec![2, 3, 1]);
        let v = vec![1, 2, 3];
        let w = p.apply(&v);
        assert_eq!(w, vec![2, 3, 1]);
    }

    #[test]
    fn test_inverse() {
        let p = Permutation::new(vec![5, 3, 1, 4, 2]);
        let q = p.inverse();
        assert_eq!(q.p, vec![3, 5, 2, 4, 1]);
    }

    #[test]
    fn test_pow() {
        let p = Permutation::new(vec![2, 1, 3]);
        let q = p.pow(3);
        assert_eq!(q.p, vec![2, 1, 3]);
    }

    #[test]
    fn test_pow2() {
        let p = Permutation::new(vec![2, 1, 3]);
        let q = p.pow(0);
        assert_eq!(q.p, vec![1, 2, 3]);
    }

    #[test]
    fn test_pow_compose() {
        let p = Permutation::new(vec![2, 1, 3]);
        let q = p.pow(2);
        let r = p.compose(&p);
        assert_eq!(q.p, r.p);
    }

    #[test]
    fn test_compress() {
        let p = Permutation::new(vec![2, 1, 3]);
        let cp = p.compress();
        let mut m = HashMap::new();
        m.insert(1, 2);
        m.insert(2, 1);
        assert_eq!(cp.m, m);
    }

    #[test]
    fn test_compute_info() {
        let p = Permutation::new(vec![2, 6, 3, 5, 4, 1]);
        let info = p.compute_info();
        assert_eq!(info.cycles, vec![vec![1, 2, 6], vec![3], vec![4, 5]]);
        assert_eq!(info.signum, false);
    }

    #[test]
    fn test_get_permutation() {
        let source = vec![1, 2, 3];
        let target = vec![3, 1, 2];
        let p = get_permutation(&source, &target);
        assert_eq!(p.p, vec![2, 3, 1]);
    }

    #[test]
    fn test_display_permutation_info() {
        let p = Permutation::new(vec![2, 6, 3, 5, 4, 1]);
        let info = p.compute_info();
        assert_eq!(format!("{}", info), "[[1, 2, 6], [3], [4, 5]] (odd)");
    }

    #[test]
    fn test_parse_permutation_from_cycle() {
        let cycle_str = "(2,3)(4,5)";
        let p = Permutation::parse_permutation_from_cycle(cycle_str, 5);
        assert_eq!(p.p, vec![1, 3, 2, 5, 4]);
    }

    #[test]
    fn test_decompose() {
        let p = Permutation::new(vec![2, 3, 1]); // (1,2,3)
        let t = vec![
            Permutation::new(vec![1, 3, 2]),
            Permutation::new(vec![3, 2, 1]),
        ];
        let result = decompose(&p.compute_info(), &t, 5);
        assert_eq!(result.is_some(), true);
        let result = result.unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].p, vec![3, 2, 1]);
        assert_eq!(result[1].p, vec![1, 3, 2]);
    }
}

#[cfg(test)]
mod compressed_permutation_tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_new() {
        let m = HashMap::new();
        let cp = CompressedPermutation::new(m);
        assert_eq!(cp.m, HashMap::new());
    }

    #[test]
    fn test_display() {
        let mut m = HashMap::new();
        m.insert(1, 2);
        m.insert(2, 3);
        let cp = CompressedPermutation::new(m);
        assert_eq!(format!("{}", cp), "{1: 2, 2: 3, }");
    }

    #[test]
    fn test_identity() {
        let cp = CompressedPermutation::identity();
        assert_eq!(cp.m, HashMap::new());
    }

    #[test]
    fn test_get() {
        let mut m = HashMap::new();
        m.insert(1, 2);
        m.insert(2, 3);
        let cp = CompressedPermutation::new(m);
        assert_eq!(cp.get(1), 2);
        assert_eq!(cp.get(2), 3);
        assert_eq!(cp.get(3), 3);
    }

    #[test]
    fn test_compose() {
        let mut m = HashMap::new();
        m.insert(1, 2);
        m.insert(2, 3);
        m.insert(3, 1);
        let cp = CompressedPermutation::new(m);
        let mut m = HashMap::new();
        m.insert(2, 3);
        m.insert(3, 2);
        let cp2 = CompressedPermutation::new(m);
        let cp3 = cp.compose(&cp2);
        let mut m = HashMap::new();
        m.insert(1, 2);
        m.insert(2, 1);
        m.insert(3, 3);
        assert_eq!(cp3.m, m);
    }

    #[test]
    fn test_inverse() {
        let mut m = HashMap::new();
        m.insert(1, 2);
        m.insert(2, 3);
        let cp = CompressedPermutation::new(m);
        let cp2 = cp.inverse();
        let mut m = HashMap::new();
        m.insert(2, 1);
        m.insert(3, 2);
        assert_eq!(cp2.m, m);
    }
}
