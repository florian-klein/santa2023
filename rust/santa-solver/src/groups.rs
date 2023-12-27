use crate::permutation::Permutation;
use std::collections::HashMap;
use std::collections::{HashSet, VecDeque};
use std::str::FromStr;

pub struct PermutationGroupIterator<'a> {
    frontier: VecDeque<(String, Permutation)>,
    visited: HashSet<Permutation>,
    queue: VecDeque<(String, Permutation)>,
    gen_to_str: &'a HashMap<Permutation, String>,
}

impl<'a> PermutationGroupIterator<'a> {
    pub fn new(gen_to_str: &'a HashMap<Permutation, String>) -> Self {
        let mut frontier = VecDeque::new();
        // get a key from gen_to_str and its length
        let (key, _) = gen_to_str.iter().next().unwrap();
        let identity = Permutation::identity(key.len());
        frontier.push_back((String::new(), identity.clone()));

        Self {
            frontier,
            visited: HashSet::new(),
            queue: VecDeque::new(),
            gen_to_str,
        }
    }
}

impl<'a> Iterator for PermutationGroupIterator<'a> {
    type Item = (String, Permutation);

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
                        new_path.push('.');
                        new_path.push_str(generator_name.as_str());
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
