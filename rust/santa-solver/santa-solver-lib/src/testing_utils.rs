use crate::permutation::Permutation;
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
        generators.push(Permutation::parse_permutation_from_cycle(
            &format!("({}, {})", n, 1),
            n,
        ));
        generators
    }

    pub fn assert_index_path_equals_permutation(
        path: &Vec<usize>,
        perm: &Permutation,
        index_to_perm: &Vec<Permutation>,
    ) -> bool {
        let mut result = Permutation::identity(perm.len());
        for i in path {
            result = result.compose(&index_to_perm[*i]);
        }
        if result != *perm {
            println!("path: {:?}", path);
            println!("result: {:?}", result);
            println!("perm: {:?}", perm);
            println!("index_to_perm: {:?}", index_to_perm);
        }
        result == *perm
    }
}
