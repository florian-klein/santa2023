use crate::permutation::Permutation;
use std::collections::HashMap;
use std::collections::{HashSet, VecDeque};

pub struct PermutationGroupIterator {
    frontier: VecDeque<(String, Permutation)>,
    visited: HashSet<Permutation>,
    queue: Vec<Permutation>,
    gen_to_str: HashMap<Permutation, String>,
}

impl<'s> PermutationGroupIterator {
    pub fn new(gen_to_str: HashMap<Permutation, String>) -> Self {
        let mut frontier = VecDeque::new();
        // get a key from gen_to_str and its length
        let (key, _) = gen_to_str.iter().next().unwrap();
        let identity = Permutation::identity(key.len());
        frontier.push_back(("".to_string(), identity.clone()));

        Self {
            frontier,
            visited: HashSet::new(),
            queue: Vec::new(),
            gen_to_str,
        }
    }
}

impl Iterator for PermutationGroupIterator {
    type Item = (String, Permutation);

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
                            .push_back((generator_name.to_string(), new_element));
                    }
                }
            } else {
                return None;
            }
        }
        let result = self.frontier.pop_front();
        if let Some(ref r) = result {
            let element_path = r.0.clone();
            let perm = r.1.clone();
            self.visited.insert(r.1.clone());
            self.queue.push(r.1.clone());
            return Some((element_path, perm));
        }
        None
    }
}
