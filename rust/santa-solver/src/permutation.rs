use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Permutation {
    p: Vec<usize>
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
}

impl Display for Permutation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.p)
    }
}

#[cfg(test)]
mod tests {
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
}