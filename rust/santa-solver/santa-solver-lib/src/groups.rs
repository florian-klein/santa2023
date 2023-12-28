use crate::permutation::{Permutation, PermutationIndex, PermutationPath};
use std::collections::HashMap;
use std::collections::{HashSet, VecDeque};
use crate::permutation;

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
        // get a key from gen_to_str and its length
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
                        self.frontier
                            .push_back((new_path, new_element));
                    }
                }
            } else {
                return None;
            }
        }
        let result = self.frontier.pop_front();
        if result.is_none() {
            return None;
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
                        self.frontier
                            .push_back((new_element, new_path));
                    }
                }
            } else {
                return None;
            }
        }
        let result = self.frontier.pop_front();
        if result.is_none() {
            return None;
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
            Permutation::new(vec![1, 3, 2, 4]),
            Permutation::new(vec![1, 2, 4, 3]),
        ];
        let mut iterator = DepthLimitedPermutationGroupIterator::new(&generators, 5);
        let mut result = iterator.next();
        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap(), (Permutation::new(vec![1, 3, 2, 4]), vec![0]));
    }
}