use crate::permutation::Permutation;
use crate::permutation::PermutationIndex;
use std::collections::HashMap;
pub struct TestingUtils {}

impl TestingUtils {
    pub fn cycle_str_to_perm(cycle_str: &str, n: usize) -> Permutation {
        Permutation::parse_permutation_from_cycle(cycle_str, n)
    }

    pub fn get_s_n_generators(n: usize) -> Vec<Permutation> {
        let mut generators = Vec::new();
        for i in 1..n {
            generators.push(Permutation::parse_permutation_from_cycle(
                &format!("({}, {})", i, i + 1),
                n,
            ));
        }
        generators
    }

    pub fn get_generator_to_perm_index_map_s_n(n: usize) -> HashMap<Permutation, PermutationIndex> {
        let mut result = HashMap::new();
        let generators = TestingUtils::get_s_n_generators(n);
        for (i, generator) in generators.iter().enumerate() {
            result.insert(generator.clone(), i);
        }
        result
    }

    pub fn get_perm_index_to_generator_map_s_n(n: usize) -> HashMap<PermutationIndex, Permutation> {
        let mut result = HashMap::new();
        let generators = TestingUtils::get_s_n_generators(n);
        for (i, generator) in generators.iter().enumerate() {
            result.insert(i, generator.clone());
        }
        result
    }

    pub fn get_index_to_perm_vec_s_n(n: usize) -> Vec<Permutation> {
        let mut result = Vec::new();
        let generators = TestingUtils::get_s_n_generators(n);
        for generator in generators {
            result.push(generator);
        }
        result
    }

    pub fn assert_cycle_list_is_c_cycle(cycle_list: Vec<Vec<usize>>, c: usize) -> () {
        let mut result = true;
        for cycle in cycle_list {
            if cycle.len() == 1 {
                continue;
            }
            if cycle.len() != c {
                result = false;
                break;
            }
        }
        assert!(result);
    }

    pub fn assert_index_path_equals_permutation_using_hashmap(
        path: &Vec<usize>,
        perm: &Permutation,
        perm_to_index: &HashMap<Permutation, PermutationIndex>,
    ) -> () {
        let mut result = Permutation::identity(perm.len());
        for i in path {
            // apply index_to_perm to resut
            let perm = perm_to_index
                .iter()
                .find(|(_, index)| **index == *i)
                .unwrap()
                .0;
            result = perm.compose(&result);
        }
        if result != *perm {
            println!("path: {:?}", path);
            println!("result: {:?}", result);
            println!("perm: {:?}", perm);
        }
        assert!(result == *perm);
    }

    pub fn assert_index_path_equals_permutation(
        path: &Vec<usize>,
        perm: &Permutation,
        index_to_perm: &Vec<Permutation>,
    ) -> () {
        let mut result = Permutation::identity(perm.len());
        for i in path {
            // apply index_to_perm to resut
            let perm = &index_to_perm[*i];
            result = perm.compose(&result);
        }
        if result != *perm {
            println!("path: {:?}", path);
            println!("result: {:?}", result);
            println!("perm: {:?}", perm);
        }
        assert!(result == *perm);
    }
}
