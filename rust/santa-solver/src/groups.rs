use crate::minkwitz::PermAndWord;
use crate::permutation::{CompressedPermutation, Permutation, PermutationIndex, PermutationPath};
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

pub struct PermutationGroupPermAndWordIterator<'a> {
    frontier: VecDeque<PermAndWord>,
    visited: HashSet<PermAndWord>,
    queue: VecDeque<PermAndWord>,
    generators: &'a HashSet<PermAndWord>,
}

pub struct DepthLimitedPermutationGroupIterator<'a> {
    frontier: VecDeque<(Permutation, Vec<usize>)>,
    visited: bloomfilter::Bloom<Permutation>,
    items_inserted: usize,
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
                    if !self.visited.contains(&new_element) {
                        let generator_name = self.gen_to_str.get(generator).unwrap();
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

impl<'a> PermutationGroupPermAndWordIterator<'a> {
    pub fn new(generators: &'a HashSet<PermAndWord>) -> Self {
        let mut frontier = VecDeque::new();
        let key = generators.iter().next().unwrap();
        let identity = Permutation::identity(key.perm.len());
        frontier.push_back(PermAndWord::new(identity.clone(), Vec::new()));

        Self {
            frontier,
            visited: HashSet::new(),
            queue: VecDeque::new(),
            generators,
        }
    }
}

impl<'a> Iterator for PermutationGroupPermAndWordIterator<'a> {
    type Item = PermAndWord;

    fn next(&mut self) -> Option<Self::Item> {
        if self.frontier.is_empty() {
            if !self.queue.is_empty() {
                let perm_and_word = self.queue.pop_front().unwrap();
                // iterate over gen_to_str keys
                for generator in self.generators {
                    let new_element = generator.compose(&perm_and_word);
                    if !self.visited.contains(&new_element) {
                        self.frontier.push_back(new_element);
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
        let perm_and_word = result.unwrap();

        self.visited.insert(perm_and_word.clone());
        self.queue.push_back(perm_and_word.clone());
        return Some(perm_and_word);
    }
}

impl<'a> DepthLimitedPermutationGroupIterator<'a> {
    pub fn new(generators: &'a Vec<Permutation>, max_depth: usize) -> Self {
        let mut queue = VecDeque::new();
        let identity = Permutation::identity(generators[0].len());
        queue.push_back((identity, Vec::<usize>::new()));

        Self {
            frontier: VecDeque::new(),
            visited: bloomfilter::Bloom::new_for_fp_rate(1_000_000, 0.0000001), //TODO adjust
            queue,
            generators,
            current_depth: 0,
            items_inserted: 0,
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
                    if !self.visited.check(&new_element) {
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

        self.visited.set(&element_perm);
        self.items_inserted += 1;
        if self.items_inserted % 10000 == 0 {
            info!(
                "Visited {} elements in DepthLimitedPermutationGroupIterator",
                self.items_inserted
            );
        }
        self.queue.push_back((element_perm.clone(), path.clone()));
        if path.len() > self.current_depth {
            self.current_depth = path.len();
        }
        return Some((element_perm, path));
    }
}

pub struct DepthLimitedPermutationGroupIteratorCompressed<'a> {
    frontier: VecDeque<(CompressedPermutation, Vec<usize>)>,
    visited: bloomfilter::Bloom<CompressedPermutation>,
    items_inserted: usize,
    queue: VecDeque<(CompressedPermutation, Vec<usize>)>,
    generators: &'a Vec<CompressedPermutation>,
    current_depth: usize,
    max_depth: usize,
}

impl<'a> DepthLimitedPermutationGroupIteratorCompressed<'a> {
    pub fn new(generators: &'a Vec<CompressedPermutation>, max_depth: usize) -> Self {
        let mut queue = VecDeque::new();
        let identity = CompressedPermutation::identity(generators[0].len());
        queue.push_back((identity, Vec::<usize>::new()));

        Self {
            frontier: VecDeque::new(),
            visited: bloomfilter::Bloom::new_for_fp_rate(1_000_000, 0.0000001), //TODO adjust
            queue,
            generators,
            current_depth: 0,
            items_inserted: 0,
            max_depth,
        }
    }
}

impl<'a> Iterator for DepthLimitedPermutationGroupIteratorCompressed<'a> {
    type Item = (CompressedPermutation, Vec<usize>);

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
                    if !self.visited.check(&new_element) {
                        let mut new_path = element_path.clone();
                        new_path.push(i);
                        self.frontier.push_back((new_element, new_path));
                    }
                }
            } else {
                error!("Error in DepthLimitedCompressedPermutationGroupIterator: Queue is empty");
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

        self.visited.set(&element_perm);
        self.items_inserted += 1;
        if self.items_inserted % 100000 == 0 {
            info!(
                "Visited {} elements in DepthLimitedCompressedPermutationGroupIterator",
                self.items_inserted
            );
        }
        self.queue.push_back((element_perm.clone(), path.clone()));
        if path.len() > self.current_depth {
            self.current_depth = path.len();
        }
        return Some((element_perm, path));
    }
}

pub struct IterativeDeepeningGroupGenerator<'a> {
    frontier: VecDeque<(Permutation, Vec<usize>)>,
    visited: bloomfilter::Bloom<Permutation>,
    stack: Vec<(Permutation, Vec<usize>, usize)>,
    generators: &'a Vec<Permutation>,
    current_depth: usize,
    items_inserted: usize,
    max_depth: usize,
}

impl<'a> IterativeDeepeningGroupGenerator<'a> {
    pub fn new(generators: &'a Vec<Permutation>, max_depth: usize) -> Self {
        let mut queue = VecDeque::new();
        let identity = Permutation::identity(generators[0].len());
        queue.push_back((identity, Vec::<usize>::new()));
        let gen_length = generators[0].len();

        Self {
            frontier: VecDeque::new(),
            visited: bloomfilter::Bloom::new_for_fp_rate(1_000_000, 0.0000001), // TODO adjust
            stack: Vec::new(),
            generators,
            current_depth: 0,
            items_inserted: 0,
            max_depth: gen_length ^ max_depth,
        }
    }
}

impl<'a> Iterator for IterativeDeepeningGroupGenerator<'a> {
    type Item = (Permutation, Vec<usize>);

    fn next(&mut self) -> Option<Self::Item> {
        while !self.stack.is_empty() || !self.frontier.is_empty() {
            if self.stack.is_empty() && self.frontier.is_empty() {
                break;
            }

            if self.stack.is_empty() {
                let (element_perm, element_path) = self.frontier.pop_front().unwrap();
                // Enumerate generators
                for (i, generator) in self.generators.iter().enumerate() {
                    let new_element = generator.compose(&element_perm);
                    if !self.visited.check(&new_element) {
                        let mut new_path = element_path.clone();
                        new_path.push(i);
                        self.stack.push((new_element, new_path, 1));
                    }
                }
            }

            let (element_perm, path, depth) = self.stack.pop().unwrap();

            if depth > self.current_depth {
                self.current_depth = depth;
            }

            self.visited.set(&element_perm);
            self.items_inserted += 1;
            if self.items_inserted % 100000 == 0 {
                println!(
                    "Visited {} elements in IterativeDeepeningGroupGenerator",
                    self.items_inserted
                );
            }

            if depth < self.max_depth {
                for (i, generator) in self.generators.iter().enumerate() {
                    let new_element = generator.compose(&element_perm);
                    if !self.visited.check(&new_element) {
                        let mut new_path = path.clone();
                        new_path.push(i);
                        self.stack.push((new_element, new_path, depth + 1));
                    }
                }
            }

            return Some((element_perm, path));
        }

        info!("Reached max depth of {} in Group Iterator", self.max_depth);
        None
    }
}

/*
* Iterative Deepening Iterator for Compressed Permutations
*/

pub struct IterativeDeepeningCompressed<'a> {
    frontier: VecDeque<(CompressedPermutation, Vec<usize>)>,
    visited: bloomfilter::Bloom<CompressedPermutation>,
    stack: Vec<(CompressedPermutation, Vec<usize>, usize)>,
    generators: &'a Vec<CompressedPermutation>,
    current_depth: usize,
    items_inserted: usize,
    max_depth: usize,
}

impl<'a> IterativeDeepeningCompressed<'a> {
    pub fn new(generators: &'a Vec<CompressedPermutation>, max_depth: usize) -> Self {
        let mut queue = VecDeque::new();
        let identity = CompressedPermutation::identity(generators[0].len());
        queue.push_back((identity, Vec::<usize>::new()));

        Self {
            frontier: VecDeque::new(),
            visited: bloomfilter::Bloom::new_for_fp_rate(1_000_000, 0.0000001), // TODO adjust
            stack: Vec::new(),
            generators,
            current_depth: 0,
            items_inserted: 0,
            max_depth,
        }
    }
}

impl<'a> Iterator for IterativeDeepeningCompressed<'a> {
    type Item = (CompressedPermutation, Vec<usize>);

    fn next(&mut self) -> Option<Self::Item> {
        while !self.stack.is_empty() || !self.frontier.is_empty() {
            if self.stack.is_empty() && self.frontier.is_empty() {
                break;
            }

            if self.stack.is_empty() {
                let (element_perm, element_path) = self.frontier.pop_front().unwrap();
                // Enumerate generators
                for (i, generator) in self.generators.iter().enumerate() {
                    let new_element = generator.compose(&element_perm);
                    if !self.visited.check(&new_element) {
                        let mut new_path = element_path.clone();
                        new_path.push(i);
                        self.stack.push((new_element, new_path, 1));
                    }
                }
            }

            let (element_perm, path, depth) = self.stack.pop().unwrap();

            if depth > self.current_depth {
                self.current_depth = depth;
            }

            self.visited.set(&element_perm);
            self.items_inserted += 1;
            if self.items_inserted % 100000 == 0 {
                println!(
                    "Visited {} elements in IterativeDeepeningCompressed",
                    self.items_inserted
                );
            }

            if depth < self.max_depth {
                for (i, generator) in self.generators.iter().enumerate() {
                    let new_element = generator.compose(&element_perm);
                    if !self.visited.check(&new_element) {
                        let mut new_path = path.clone();
                        new_path.push(i);
                        self.stack.push((new_element, new_path, depth + 1));
                    }
                }
            }

            return Some((element_perm, path));
        }

        info!("Reached max depth of {} in Group Iterator", self.max_depth);
        None
    }
}

#[cfg(test)]
mod permutation_group_iterator_tests {
    use super::*;
    use crate::permutation::Permutation;

    #[test]
    fn test_permutation_group_iterator() {
        let generators = vec![
            Permutation::parse_permutation_from_cycle("(1,2)", 3),
            Permutation::parse_permutation_from_cycle("(2,3)", 3),
        ];
        let mut gen_to_index = HashMap::new();
        gen_to_index.insert(generators[0].clone(), 0);
        gen_to_index.insert(generators[1].clone(), 1);
        let iterator = PermutationGroupIterator::new(&gen_to_index);
        for (path, perm) in iterator {
            println!("{:?} -> {}", path, perm);
        }
    }

    // #[test]
    fn test_permutation_group_iterator_perm_and_word() {
        let generators: HashSet<PermAndWord> = vec![
            PermAndWord::new(
                Permutation::parse_permutation_from_cycle("(1,2)", 3),
                vec![0],
            ),
            PermAndWord::new(
                Permutation::parse_permutation_from_cycle("(2,3)", 3),
                vec![1],
            ),
        ]
        .into_iter()
        .collect();
        let iterator = PermutationGroupPermAndWordIterator::new(&generators);
        let mut counter = 0;
        for perm_and_word in iterator {
            println!("{:?}", perm_and_word);
            counter += 1;
        }
        assert_eq!(counter, 6);
    }

    #[test]
    fn test_permutation_group_iterator_larger() {
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
}

#[cfg(test)]
mod depth_limited_permutation_group_iterator_tests {
    use super::*;
    use crate::permutation::Permutation;
    use crate::testing_utils::TestingUtils;

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
        let mut seen_perms = HashSet::new();
        while let Some((path, perm)) = iterator.next() {
            TestingUtils::assert_index_path_equals_permutation(&path.arr, &perm, &generators);
            counter += 1;
            if seen_perms.contains(&perm) {
                panic!("Duplicate permutation found: {}", perm);
            } else {
                seen_perms.insert(perm);
            }
        }
        assert_eq!(counter, 120);
    }
}
