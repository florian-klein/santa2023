use crate::permutation::{Permutation, PermutationIndex, PermutationPath};
use crate::testing_utils::TestingUtils;
use log::error;
use log::info;
use std::collections::HashMap;
use std::collections::{HashSet, VecDeque};

pub struct PermutationGroupIterator<'a> {
    frontier: VecDeque<(PermutationPath, Permutation)>,
    visited: HashSet<Permutation>,
    queue: VecDeque<(PermutationPath, Permutation)>,
    gen_to_str: &'a HashMap<Permutation, PermutationIndex>,
}

pub struct DepthLimitedPermutationGroupIterator<'a> {
    frontier: VecDeque<(Permutation, Vec<usize>)>,
    visited: HashSet<Permutation>,
    queue: VecDeque<(Permutation, Vec<usize>)>,
    generators: &'a Vec<Permutation>,
    current_depth: usize,
    max_depth: usize,
}

impl<'a> PermutationGroupIterator<'a> {
    pub fn new(gen_to_str: &'a HashMap<Permutation, PermutationIndex>) -> Self {
        let mut frontier = VecDeque::new();
        let (key, _) = gen_to_str.iter().next().unwrap();
        let identity = Permutation::identity(key.len());
        frontier.push_back((PermutationPath::default(), identity.clone()));

        Self {
            frontier,
            visited: HashSet::new(),
            queue: VecDeque::new(),
            gen_to_str,
        }
    }
}

impl<'a> Iterator for PermutationGroupIterator<'a> {
    type Item = (PermutationPath, Permutation);

    fn next(&mut self) -> Option<Self::Item> {
        if self.frontier.is_empty() {
            if !self.queue.is_empty() {
                let (element_path, element_perm) = self.queue.pop_front().unwrap();
                // iterate over gen_to_str keys
                for generator in self.gen_to_str.keys() {
                    let new_element = generator.compose(&element_perm);
                    let generator_name = self.gen_to_str.get(generator).unwrap();
                    if !self.visited.contains(&new_element) {
                        let mut new_path = element_path.clone();
                        new_path.push(*generator_name);
                        self.frontier.push_back((new_path, new_element));
                    }
                }
            } else {
                error!("Error in PermutationGroupIterator: Queue is empty");
                return None;
            }
        }
        let result = self.frontier.pop_front();
        if self.queue.is_empty() && result.is_none() {
            info!("PermutationGroupIterator Frontier is now empty, proceeding ...");
            return None;
        } else if result.is_none() {
            return self.next();
        }
        let (element_path, perm) = result.unwrap();

        self.visited.insert(perm.clone());
        self.queue.push_back((element_path.clone(), perm.clone()));
        return Some((element_path, perm));
    }
}

impl<'a> DepthLimitedPermutationGroupIterator<'a> {
    pub fn new(generators: &'a Vec<Permutation>, max_depth: usize) -> Self {
        let mut queue = VecDeque::new();
        let identity = Permutation::identity(generators[0].len());
        queue.push_back((identity, Vec::<usize>::new()));

        Self {
            frontier: VecDeque::new(),
            visited: HashSet::new(),
            queue,
            generators,
            current_depth: 0,
            max_depth,
        }
    }
}

impl<'a> Iterator for DepthLimitedPermutationGroupIterator<'a> {
    type Item = (Permutation, Vec<usize>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_depth == self.max_depth {
            info!("Reached max depth of {} in Group Iterator", self.max_depth);
            return None;
        }
        if self.frontier.is_empty() {
            if !self.queue.is_empty() {
                let (element_perm, element_path) = self.queue.pop_front().unwrap();
                // Enumerate generators
                for (i, generator) in self.generators.iter().enumerate() {
                    let new_element = generator.compose(&element_perm);
                    if !self.visited.contains(&new_element) {
                        let mut new_path = element_path.clone();
                        new_path.push(i);
                        self.frontier.push_back((new_element, new_path));
                    }
                }
            } else {
                error!("Error in DepthLimitedPermutationGroupIterator: Queue is empty");
                return None;
            }
        }
        let result = self.frontier.pop_front();
        if self.queue.is_empty() && result.is_none() {
            info!("Group Iterator Frontier is now empty, proceeding ...");
            return None;
        } else if result.is_none() {
            return self.next();
        }

        let (element_perm, path) = result.unwrap();

        self.visited.insert(element_perm.clone());
        self.queue.push_back((element_perm.clone(), path.clone()));
        if path.len() > self.current_depth {
            self.current_depth = path.len();
        }
        return Some((element_perm, path));
    }
}

#[cfg(test)]
mod depth_limited_permutation_group_iterator_tests {
    use super::*;
    use crate::permutation::Permutation;

    #[test]
    fn test_depth_limited_permutation_group_iterator() {
        let generators = vec![
            Permutation::parse_permutation_from_cycle("(1,2)", 3),
            Permutation::parse_permutation_from_cycle("(2,3)", 3),
        ];
        let mut iterator = DepthLimitedPermutationGroupIterator::new(&generators, 10);
        assert_eq!(
            iterator.next().unwrap(),
            (Permutation::new(vec![2, 1, 3]), vec![0])
        );
        assert_eq!(
            iterator.next().unwrap(),
            (Permutation::new(vec![1, 3, 2]), vec![1])
        );
        assert_eq!(
            iterator.next().unwrap(),
            (Permutation::new(vec![1, 2, 3]), vec![0, 0])
        );
        assert_eq!(
            iterator.next().unwrap(),
            (Permutation::new(vec![3, 1, 2]), vec![0, 1])
        );
        assert_eq!(
            iterator.next().unwrap(),
            (Permutation::new(vec![2, 3, 1]), vec![1, 0])
        );
        assert_eq!(
            iterator.next().unwrap(),
            (Permutation::new(vec![3, 2, 1]), vec![0, 1, 0])
        );
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn test_depth_limited_permutation_group_iterator_larger() {
        let generators = TestingUtils::get_s_n_generators(5);
        println!("generators: {:?}", generators);
        let mut iterator = DepthLimitedPermutationGroupIterator::new(&generators, 100);
        assert_eq!(
            iterator.next().unwrap(),
            (Permutation::new(vec![2, 1, 3, 4, 5]), vec![0])
        );
        assert_eq!(
            iterator.next().unwrap(),
            (Permutation::new(vec![1, 3, 2, 4, 5]), vec![1])
        );
        assert_eq!(
            iterator.next().unwrap(),
            (Permutation::new(vec![1, 2, 4, 3, 5]), vec![2])
        );
        let mut counter = 3;
        while let Some((perm, path)) = iterator.next() {
            TestingUtils::assert_index_path_equals_permutation(&path, &perm, &generators);
            counter += 1;
        }

        // assert_eq!(last_perm, Permutation::new(vec![5, 4, 3, 2, 1]));
        // assert_eq!(last_path, vec![0, 1, 0, 2, 1, 0, 3, 2, 1, 0]);
        // symmetric group of 5 elements has 120 elements
        assert_eq!(counter, 120);
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn test_perm_group_iterator() {
        let generators = TestingUtils::get_s_n_generators(3);
        let mut gen_to_index = HashMap::new();
        gen_to_index.insert(generators[0].clone(), 0);
        gen_to_index.insert(generators[1].clone(), 1);
        let mut iterator = PermutationGroupIterator::new(&gen_to_index);
        let mut counter = 0;
        while let Some((path, perm)) = iterator.next() {
            TestingUtils::assert_index_path_equals_permutation(&path.arr, &perm, &generators);
            counter += 1;
        }
        assert_eq!(counter, 6);
    }

    #[test]
    fn test_perm_group_iterator_larger() {
        let generators = TestingUtils::get_s_n_generators(5);
        let mut gen_to_index = HashMap::new();
        gen_to_index.insert(generators[0].clone(), 0);
        gen_to_index.insert(generators[1].clone(), 1);
        gen_to_index.insert(generators[2].clone(), 2);
        gen_to_index.insert(generators[3].clone(), 3);
        let mut iterator = PermutationGroupIterator::new(&gen_to_index);
        let mut counter = 0;
        while let Some((path, perm)) = iterator.next() {
            TestingUtils::assert_index_path_equals_permutation(&path.arr, &perm, &generators);
            counter += 1;
        }
        assert_eq!(counter, 120);
    }
}
