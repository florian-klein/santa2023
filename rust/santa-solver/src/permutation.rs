use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Permutation {
    p: Vec<usize>
}

#[derive(Debug, Clone)]
pub struct CompressedPermutation {
    m: HashMap<usize, usize>
}

impl Permutation {
    pub fn new(p: Vec<usize>) -> Permutation {
        Permutation { p }
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
}

impl Display for Permutation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.p)
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

#[cfg(test)]
mod permutation_tests {
    use super::*;

    #[test]
    fn test_new() {
        let p = Permutation::new(vec![1, 2, 3]);
        assert_eq!(p.p, vec![1, 2, 3]);
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
}

#[cfg(test)]
mod compressed_permutation_tests {
    use std::collections::HashMap;
    use super::*;

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