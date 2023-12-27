use std::collections::{VecDeque, HashSet};
use crate::permutation::Permutation;

pub struct PermutationGroupIterator<'s> {
    frontier: VecDeque<Permutation>,
    visited: HashSet<Permutation>,
    queue: Vec<Permutation>,
    generators: &'s Vec<Permutation>,
}

impl PermutationGroupIterator<'_> {
    pub fn new(generators: &Vec<Permutation>) -> Self {
        let mut frontier = VecDeque::new();
        let identity = Permutation::identity(generators[0].len());
        frontier.push_back(identity.clone());

        Self {
            frontier,
            visited: HashSet::new(),
            queue: Vec::new(),
            generators,
        }
    }
}

impl Iterator for PermutationGroupIterator<'_> {
    type Item = Permutation;

    fn next(&mut self) -> Option<Self::Item> {
        if self.frontier.is_empty() {
            if !self.queue.is_empty() {
                let element= self.queue.remove(0);
                for generator in self.generators {
                    let new_element = generator.compose(&element);
                    if !self.visited.contains(&new_element) {
                        self.frontier.push_back(new_element);
                    }
                }
            } else {
                return None;
            }
        }
        let result = self.frontier.pop_front();
        if let Some(ref r) = result {
            self.visited.insert(r.clone());
            self.queue.push(r.clone());
            return Some(r.clone());
        }
        None
    }
}
