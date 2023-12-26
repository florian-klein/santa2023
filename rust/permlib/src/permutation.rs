use std::ops::Mul;
use std::ops::MulAssign;

#[derive(Debug)]
#[derive(Clone)]
pub struct Permutation {
    elements: Vec<usize>,
}

impl Permutation {
    pub fn new(elements: Vec<usize>) -> Permutation {
        Permutation { elements }
    }

    pub fn apply_to_other_permutation(&self, permutation: &Permutation) -> Permutation {
        let mut result = vec![0; self.elements.len()];

        for (i, &element) in self.elements.iter().enumerate() {
            result[i] = permutation.elements[element];
        }

        Permutation::new(result)
    }
    
    // example perm = (0, 1)(2); perm.apply_to_single_element(0) = 1
    pub fn apply_to_single_element(&self, element: usize) -> usize {
        self.elements[element]
    }

    pub fn inverse(&self) -> Permutation {
        let mut result = vec![0; self.elements.len()];

        for (i, &element) in self.elements.iter().enumerate() {
            result[element] = i;
        }

        Permutation::new(result)
    }

}

impl Mul for Permutation {
    type Output = Permutation;

    // Combine two permutations by composing their actions
    fn mul(self, rhs: Permutation) -> Permutation {
        let combined_elements: Vec<usize> = rhs.elements.iter().map(|&i| self.elements[i]).collect();
        Permutation::new(combined_elements)
    }
}

impl MulAssign for Permutation {
    fn mul_assign(&mut self, rhs: Permutation) {
        self.elements = rhs.elements.iter().map(|&i| self.elements[i]).collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_to_other_permutation() {
        let perm1 = Permutation::new(vec![2, 0, 1]);
        let perm2 = Permutation::new(vec![1, 2, 0]);
        assert_eq!(perm1.apply_to_other_permutation(&perm2).elements, vec![0, 1, 2]);
    }

    #[test]
    fn test_apply_to_single_element() {
        let perm1 = Permutation::new(vec![2, 0, 1]);
        assert_eq!(perm1.apply_to_single_element(0), 2);
    }

    #[test]
    fn test_inverse() {
        let perm1 = Permutation::new(vec![2, 0, 1]);
        assert_eq!(perm1.inverse().elements, vec![1, 2, 0]);
        assert_eq!(perm1.inverse().inverse().elements, vec![2, 0, 1]);
    }

    #[test]
    fn test_mul() {
        let perm1 = Permutation::new(vec![2, 0, 1]);
        let perm2 = Permutation::new(vec![1, 2, 0]);
        assert_eq!((perm1 * perm2).elements, vec![0, 1, 2]);
    }

    #[test]
    fn test_mul_assign() {
        let mut perm1 = Permutation::new(vec![2, 0, 1]);
        let perm2 = Permutation::new(vec![1, 2, 0]);
        perm1 *= perm2;
        assert_eq!(perm1.elements, vec![0, 1, 2]);
    }

    #[test]
    fn test_new() {
        let perm1 = Permutation::new(vec![2, 0, 1]);
        assert_eq!(perm1.elements, vec![2, 0, 1]);
    }

    #[test]
    fn test_clone() {
        let perm1 = Permutation::new(vec![2, 0, 1]);
        let perm2 = perm1.clone();
        assert_eq!(perm1.elements, perm2.elements);
    }

}

