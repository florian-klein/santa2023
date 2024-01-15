use crate::permutation::Permutation;

pub struct DepthLimitedPermutationGroupIterator<'a> {
    stack: Vec<(Permutation, Vec<usize>, usize)>,
    generators: &'a Vec<Permutation>,
    max_depth: usize,
}

impl<'a> DepthLimitedPermutationGroupIterator<'a> {
    pub fn new(generators: &'a Vec<Permutation>, max_depth: usize) -> Self {
        let mut stack = Vec::new();
        let identity = Permutation::identity(generators[0].len());
        stack.push((identity, vec![], 0));

        Self {
            stack,
            generators,
            max_depth,
        }
    }
}

impl<'a> Iterator for DepthLimitedPermutationGroupIterator<'a> {
    type Item = (Permutation, Vec<usize>, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.stack.is_empty() {
            return None;
        }
        let (mut perm, mut path, mut next_index) = self.stack.pop().unwrap();
        if next_index + 1 < self.generators.len() {
            self.stack
                .push((perm.clone(), path.clone(), next_index + 1));
        }
        while path.len() < self.max_depth {
            let new_perm = self.generators[next_index].compose(&perm);
            let new_path = {
                let mut new_path = path.clone();
                new_path.push(next_index);
                new_path
            };
            self.stack.push((new_perm.clone(), new_path.clone(), 1));
            perm = new_perm;
            path = new_path;
            next_index = 0;
        }
        return self.stack.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing_utils::TestingUtils;

    #[test]
    fn test_depth_limited_permutation_group_iterator() {
        let generators = TestingUtils::get_s_n_generators(4);
        let mut group_iterator = DepthLimitedPermutationGroupIterator::new(&generators, 4);
        let checker = TestingUtils::get_index_to_perm_vec_s_n(4);
        let mut group = Vec::new();
        while let Some((perm, path, _)) = group_iterator.next() {
            TestingUtils::assert_index_path_equals_permutation(&path, &perm, &checker);
            group.push((perm, path));
        }
        assert_eq!(group.len(), 81);
    }
}
