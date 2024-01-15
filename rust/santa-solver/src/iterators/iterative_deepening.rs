use crate::permutation::Permutation;

use super::depth_limited::DepthLimitedPermutationGroupIterator;

pub struct IterativeDeepeningGroupGenerator<'a> {
    generators: &'a Vec<Permutation>,
    current_depth: usize,
    items_inserted: usize,
    cur_iterator: Option<DepthLimitedPermutationGroupIterator<'a>>,
    max_depth: usize,
}

impl<'a> IterativeDeepeningGroupGenerator<'a> {
    pub fn new(generators: &'a Vec<Permutation>, max_depth: usize) -> Self {
        let cur_iterator = DepthLimitedPermutationGroupIterator::new(generators, 1);
        Self {
            generators,
            current_depth: 1,
            items_inserted: 0,
            cur_iterator: Some(cur_iterator),
            max_depth,
        }
    }
}

impl<'a> Iterator for IterativeDeepeningGroupGenerator<'a> {
    type Item = (Permutation, Vec<usize>);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(cur_iterator) = &mut self.cur_iterator {
            if let Some((perm, path, _)) = cur_iterator.next() {
                self.items_inserted += 1;
                return Some((perm, path));
            } else {
                self.current_depth += 1;
                if self.current_depth > self.max_depth {
                    return None;
                }
                let cur_iterator =
                    DepthLimitedPermutationGroupIterator::new(self.generators, self.current_depth);
                self.cur_iterator = Some(cur_iterator);
                return self.next();
            }
        } else {
            return None;
        }
    }
}

#[cfg(test)]
mod test_iterative_deepening_group_gen {
    use crate::testing_utils::TestingUtils;

    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_iterative_deepening_group_gen() {
        let generators = TestingUtils::get_s_n_generators(3);
        let mut group_gen = IterativeDeepeningGroupGenerator::new(&generators, 3);
        let mut group_elements = HashSet::new();
        let mut group_paths = HashSet::new();
        while let Some((element, path)) = group_gen.next() {
            group_elements.insert(element.clone());
            println!("Element: {:?}", element);
            group_paths.insert(path.clone());
            println!("Path: {:?}", path);
        }
        println!("Group elements: {:?}", group_elements);
        println!("Group paths: {:?}", group_paths);
    }
}
