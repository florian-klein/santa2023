use crate::permutation::Permutation;
use crate::permutation::PermutationIndex;
use crate::puzzle;
use crate::puzzle::PuzzleType;
use std::collections::HashMap;
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
        generators
    }

    pub fn get_generator_to_perm_index_map_s_n(n: usize) -> HashMap<Permutation, PermutationIndex> {
        let mut result = HashMap::new();
        let generators = TestingUtils::get_s_n_generators(n);
        for (i, generator) in generators.iter().enumerate() {
            result.insert(generator.clone(), i);
        }
        result
    }

    pub fn get_perm_index_to_generator_map_s_n(n: usize) -> HashMap<PermutationIndex, Permutation> {
        let mut result = HashMap::new();
        let generators = TestingUtils::get_s_n_generators(n);
        for (i, generator) in generators.iter().enumerate() {
            result.insert(i, generator.clone());
        }
        result
    }

    pub fn get_permutation_from_operation_string(
        op_str: String,
        str_to_gen: HashMap<String, Permutation>,
    ) -> Permutation {
        let n = str_to_gen.iter().next().unwrap().1.len();
        let mut result = Permutation::identity(n);
        // is in format gen1.gen2.-gen1.gen2
        let operations = op_str.split(".");
        for operation in operations {
            let next_perm = &str_to_gen.get(operation).unwrap();
            result = next_perm.compose(&result);
        }
        result
    }

    pub fn assert_permutation_equals_operation_string(
        perm: &Permutation,
        op_str: String,
        str_to_gen: HashMap<String, Permutation>,
    ) -> () {
        let perm_from_op_str =
            TestingUtils::get_permutation_from_operation_string(op_str, str_to_gen);
        if perm_from_op_str != *perm {
            println!("perm: {:?}", perm);
            println!("perm_from_op_str: {:?}", perm_from_op_str);
        }
        assert!(perm_from_op_str == *perm);
    }

    pub fn get_index_to_perm_vec_s_n(n: usize) -> Vec<Permutation> {
        let mut result = Vec::new();
        let generators = TestingUtils::get_s_n_generators(n);
        for generator in generators {
            result.push(generator);
        }
        result
    }

    pub fn assert_cycle_list_is_c_cycle(cycle_list: Vec<Vec<usize>>, c: usize) -> () {
        let mut result = true;
        for cycle in cycle_list {
            if cycle.len() == 1 {
                continue;
            }
            if cycle.len() != c {
                result = false;
                break;
            }
        }
        assert!(result);
    }

    pub fn assert_index_path_equals_permutation_using_hashmap(
        path: &Vec<usize>,
        perm: &Permutation,
        perm_to_index: &HashMap<Permutation, PermutationIndex>,
    ) -> () {
        let mut result = Permutation::identity(perm.len());
        for i in path {
            // apply index_to_perm to resut
            let perm = perm_to_index
                .iter()
                .find(|(_, index)| **index == *i)
                .unwrap()
                .0;
            result = perm.compose(&result);
        }
        if result != *perm {
            println!("path: {:?}", path);
            println!("result: {:?}", result);
            println!("perm: {:?}", perm);
        }
        assert!(result == *perm);
    }

    pub fn assert_perm_equals_op_string_for_puzzle_type(
        perm: Permutation,
        op_string: String,
        puzzle_type: PuzzleType,
    ) {
        let allowed_moves = puzzle::load_puzzle_info("./../../data/puzzle_info.csv")
            .unwrap()
            .get(&puzzle_type)
            .unwrap()
            .clone();
        let str_to_gen: HashMap<String, Permutation> = allowed_moves
            .iter()
            .map(|m| (m.name.clone(), m.permutation.clone()))
            .collect();
        TestingUtils::assert_permutation_equals_operation_string(&perm, op_string, str_to_gen);
    }

    pub fn assert_applying_sol_string_to_initial_string_results_in_target(
        initial_string: String,
        goal_string: String,
        sol_string: String,
        puzzle_type: PuzzleType,
    ) {
        let allowed_moves = puzzle::load_puzzle_info("./../../data/puzzle_info.csv")
            .unwrap()
            .get(&puzzle_type)
            .unwrap()
            .clone();
        let str_to_gen: HashMap<String, Permutation> = allowed_moves
            .iter()
            .map(|m| (m.name.clone(), m.permutation.clone()))
            .collect();
        let res_perm = TestingUtils::get_permutation_from_operation_string(sol_string, str_to_gen);
        let res_string = TestingUtils::apply_permutation_to_string(res_perm, &initial_string);
        assert!(res_string == goal_string);
    }

    pub fn apply_permutation_to_string(perm: Permutation, string: &String) -> String {
        let string_vec = string.split(";").collect::<Vec<&str>>();
        let mut result: Vec<String> = vec!["".to_string(); perm.len()];
        for i in 0..string_vec.len() {
            let index = perm.p[i] - 1;
            result[index] = string_vec[i].to_string();
        }
        result.join(";")
    }

    pub fn validate_cycles_csv(path: String, puzzle_type: PuzzleType) -> () {
        let allowed_moves = puzzle::load_puzzle_info("./../../data/puzzle_info.csv")
            .unwrap()
            .get(&puzzle_type)
            .unwrap()
            .clone();
        let str_to_gen: HashMap<String, Permutation> = allowed_moves
            .iter()
            .map(|m| (m.name.clone(), m.permutation.clone()))
            .collect();
        // open csv file and read cycles
        let mut reader = csv::Reader::from_path(path).unwrap();
        // [0, 2, 1], l.r.r, 19
        // perm, moves, num_moves
        for record in reader.records() {
            let record = record.unwrap();
            let perm = Permutation::parse_permutation_from_str_arr(record[0].to_string());
            let move_string = record[1].to_string();
            TestingUtils::assert_permutation_equals_operation_string(
                &perm,
                move_string,
                str_to_gen.clone(),
            );
        }
    }

    pub fn get_perm_from_index_path(
        path: &Vec<usize>,
        index_to_perm: &Vec<Permutation>,
    ) -> Permutation {
        let mut result = Permutation::identity(index_to_perm[0].len());
        for i in path {
            // apply index_to_perm to resut
            let perm = &index_to_perm[*i];
            result = perm.compose(&result);
        }
        result
    }

    pub fn assert_index_path_equals_permutation(
        path: &Vec<usize>,
        perm: &Permutation,
        index_to_perm: &Vec<Permutation>,
    ) -> () {
        let mut result = Permutation::identity(perm.len());
        for i in path {
            let perm = &index_to_perm[*i];
            result = perm.compose(&result);
        }
        if result != *perm {
            println!("path: {:?}", path);
            println!("result: {:?}", result);
            println!("perm: {:?}", perm);
            let composed = perm.compose(&result);
            println!("composed: {:?}", composed);
        }
        assert!(result == *perm);
    }
}

#[cfg(test)]
pub mod test {
    #[test]
    fn test_apply_perm_to_string() {
        let perm = crate::permutation::Permutation::parse_permutation_from_cycle("(1,2,3)", 3);
        let string = "a;b;c".to_string();
        let res = crate::testing_utils::TestingUtils::apply_permutation_to_string(perm, &string);
        if res != "c;a;b".to_string() {
            println!("res: {}", res);
            assert_eq!(res, "c;a;b".to_string());
        }
        assert!(res == "c;a;b".to_string());
    }
}
