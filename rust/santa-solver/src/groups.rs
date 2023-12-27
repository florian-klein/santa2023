use crate::permutation::Permutation;
use std::collections::HashMap;
use std::collections::{HashSet, VecDeque};

pub struct PermutationGroupIterator<'a> {
    frontier: VecDeque<(&'a str, Permutation)>,
    visited: HashSet<Permutation>,
    queue: Vec<Permutation>,
    gen_to_str: &'a HashMap<Permutation, String>,
}

impl<'a> PermutationGroupIterator<'a> {
    pub fn new(gen_to_str: &'a HashMap<Permutation, String>) -> Self {
        let mut frontier = VecDeque::new();
        // get a key from gen_to_str and its length
        let (key, _) = gen_to_str.iter().next().unwrap();
        let identity = Permutation::identity(key.len());
        frontier.push_back(("", identity.clone()));

        Self {
            frontier,
            visited: HashSet::new(),
            queue: Vec::new(),
            gen_to_str,
        }
    }
}

impl<'a> Iterator for PermutationGroupIterator<'a> {
    type Item = (&'a str, Permutation);

    fn next(&mut self) -> Option<Self::Item> {
        if self.frontier.is_empty() {
            if !self.queue.is_empty() {
                let element = self.queue.remove(0);
                // iterate over gen_to_str keys
                for generator in self.gen_to_str.keys() {
                    let new_element = generator.compose(&element);
                    let generator_name = self.gen_to_str.get(generator).unwrap();
                    if !self.visited.contains(&new_element) {
                        self.frontier
                            .push_back((generator_name.as_str(), new_element));
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
        self.queue.push(perm.clone());
        return Some((element_path, perm));
    }
}
