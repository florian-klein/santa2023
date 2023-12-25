use crate::permgroups;
use crate::Permutation;
use std::collections::HashSet;

#[derive(Debug)]
pub struct SchreierVector {
    vector: Vec<isize>,
    k: usize,
}

impl SchreierVector {
    /**
     * Returns indeces of the generators that are used to generate the element at index `to` using
     * the schreier vector.
     */
    // WARN!!!
    // WARN: Does not work for 0 index yet at -num is used to represent inverse action
    pub fn get_element_in_gen_decomp(&self, from: usize, generating_set: &permgroups::GeneratingSet) -> Vec<isize> {
        let mut result : Vec<isize> = vec![];
        let to = self.k;
        let mut current_index = from;
        while current_index != to {
            let current_value = self.vector[current_index];
            if current_value == -1 {
                return vec![];
            }
            let current_generator_index = if current_value % 2 == 0 {current_value / 2 - 1} else {(current_value - 1) / 2};
            let current_generator = &generating_set.generators[current_generator_index as usize];
            let current_generator_inverse = if current_value % 2 == 0 {current_generator.clone()} else {current_generator.inverse()};
            current_index = current_generator_inverse.apply_to_single_element(current_index);
            let factor = if current_value % 2 == 0 {-1} else {1};
            result.push(factor * current_generator_index);
        }
        result
    }

    pub fn find_orbit(&self) -> Vec<usize> {
        let mut result = vec![];
        // all elements that are not -1 in schreier vector are in the orbit 
        for (i, &element) in self.vector.iter().enumerate() {
            if element != -1 {
                result.push(i);
            }
        }
        result
    }

    // returns index of the coset represenative of the element
    pub fn find_new_generators(&self, generating_set: &permgroups::GeneratingSet) -> HashSet<Vec<isize>> {
        let mut result = HashSet::new();
        let orbit = self.find_orbit();
        // create map of index to gen decomp array 
        let mut index_to_gen_decomp = vec![];
        for &i in &orbit {
            index_to_gen_decomp.push(self.get_element_in_gen_decomp(i, generating_set));
        }
        for i in 0..orbit.len(){
            let left_gen = &index_to_gen_decomp[i];
            if left_gen.len() == 0 {
                continue;
            }
            for j in 0..orbit.len(){
                let right_gen = &index_to_gen_decomp[j];
                if i == j || right_gen.len() == 0 {
                    continue;
                }
                for k in 0..generating_set.generators.len(){
                    let mut new_gen_decomp = vec![];
                    for &x in &index_to_gen_decomp[i] {
                        new_gen_decomp.push(x);
                    }
                    new_gen_decomp.push(k.try_into().unwrap());
                    for &x in &index_to_gen_decomp[j] {
                        new_gen_decomp.push(x);
                    }
                    // add newgen to set 
                    result.insert(new_gen_decomp);
                }
            }
        }
        result
    }
}

fn calculate_schreier_vector(generating_set: &permgroups::GeneratingSet, permutation_length: usize, k: usize) -> SchreierVector {
    let mut v: Vec<isize> = vec![-1; permutation_length];
    v[k] = 0;

    for i in 0..permutation_length {
        if v[i] != -1 {
            let genset_len = generating_set.generators.len();
            for r in 0..genset_len {
                let g = &generating_set.generators[r];
                let j = g.apply_to_single_element(i);
                if v[j] == -1 {
                    if let Ok(vj) = (2 * r + 1).try_into() {
                        v[j] = vj;
                    } else {
                        panic!("Conversion error for value {}", 2 * r + 1);
                    }
                }
                let j = g.inverse().apply_to_single_element(i);
                if v[j] == -1 {
                    if let Ok(vj) = (2 * r + 2).try_into() {
                        v[j] = vj;
                    } else {
                        panic!("Conversion error for value {}", 2 * r + 2);
                    }
                }
            }
        }
    }
    SchreierVector { vector: v , k: k}
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_schreier_vector() {
        let perm1 = Permutation::new(vec![0, 5, 1, 2, 7, 3, 6, 9, 8, 4]); 
        let perm2 = Permutation::new(vec![0, 6, 2, 3, 4, 5, 7, 1, 8, 9]);
        let generating_set = permgroups::GeneratingSet::new(vec![perm1, perm2]);
        let schreier_vector_0 = calculate_schreier_vector(&generating_set, 10, 1);
        assert_eq!(schreier_vector_0.vector, vec![-1, 0, 2, 2, 2, 1, 3, 4, -1, 1]);
    }

    #[test]
    fn test_find_decomp_using_schreier(){
        let perm1 = Permutation::new(vec![0, 5, 1, 2, 7, 3, 6, 9, 8, 4]); 
        let perm2 = Permutation::new(vec![0, 6, 2, 3, 4, 5, 7, 1, 8, 9]);
        let generating_set = permgroups::GeneratingSet::new(vec![perm1, perm2]);
        let schreier_vector_0 = calculate_schreier_vector(&generating_set, 10, 1);
        println!("{:?}", schreier_vector_0);
        let decomp = schreier_vector_0.get_element_in_gen_decomp(9, &generating_set);
        assert_eq!(decomp, vec![0, -1]);
    }

    #[test]
    fn test_find_orbit(){
        let perm1 = Permutation::new(vec![0, 5, 1, 2, 7, 3, 6, 9, 8, 4]); 
        let perm2 = Permutation::new(vec![0, 6, 2, 3, 4, 5, 7, 1, 8, 9]);
        let generating_set = permgroups::GeneratingSet::new(vec![perm1, perm2]);
        let schreier_vector_0 = calculate_schreier_vector(&generating_set, 10, 1);
        println!("{:?}", schreier_vector_0);
        let orbit = schreier_vector_0.find_orbit();
        assert_eq!(orbit, vec![1, 2, 3, 4, 5, 6, 7, 9]);
    }

    #[test]
    fn test_find_new_generators(){
        let perm1 = Permutation::new(vec![0, 5, 6, 3, 4, 7, 8, 1, 2, 9]);
        let perm2 = Permutation::new(vec![0, 5, 3, 4, 8, 1, 6, 7, 2, 9]);
        let generating_set = permgroups::GeneratingSet::new(vec![perm1, perm2]);
        let schreier_vector_0 = calculate_schreier_vector(&generating_set, 10, 1);
        println!("{:?}", schreier_vector_0);
        assert_eq!(schreier_vector_0.find_orbit(), vec![1, 5, 7]);
        assert_eq!(schreier_vector_0.get_element_in_gen_decomp(5, &generating_set), vec![-1]);
        let new_generators = schreier_vector_0.find_new_generators(&generating_set);
        let expected : HashSet<Vec<isize>> = vec![vec![0, 1, 0]].into_iter().collect();
        assert_eq!(new_generators, expected);
    }
}









