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
        num_wildcards: usize,
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
        let sol_string_reversed = sol_string.split(".");
        let mut sol_string_reversed_vec: Vec<&str> = sol_string_reversed.collect();
        sol_string_reversed_vec.reverse();
        let joined_string = sol_string_reversed_vec.join(".");
        let res_perm =
            TestingUtils::get_permutation_from_operation_string(joined_string, str_to_gen);
        let res_string =
            TestingUtils::apply_permutation_to_string(res_perm.inverse(), &initial_string);
        let mut num_mismatches = 0;
        for (i, c) in res_string.chars().enumerate() {
            if c != goal_string.chars().nth(i).unwrap() {
                num_mismatches += 1;
            }
        }
        if num_mismatches > num_wildcards {
            println!("initial_string: \t{}", initial_string);
            println!("res_string: \t\t {}", res_string);
            println!("expected res_string: \t {}", goal_string);
            println!("num mismatches: \t {}", num_mismatches);
            println!("num wildcards: \t\t {}", num_wildcards);
            assert_eq!(0, 1);
        }
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
    use std::collections::HashMap;

    use crate::permutation::Permutation;

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

    #[test]
    fn test_get_permutation_from_operation_string() {
        let mut str_to_gen: HashMap<String, Permutation> = HashMap::new();
        let f1_perm =
            Permutation::parse_permutation_from_cycle("(3,20,22,9)(4,18,21,11)(5,7,8,6)", 24);
        let f2_perm =
            Permutation::parse_permutation_from_cycle("(1,19,24,10)(2,17,23,12)(13,14,16,15)", 24);
        str_to_gen.insert("f0".to_string(), f1_perm.clone());
        str_to_gen.insert("f1".to_string(), f2_perm.clone());
        let op_string = "f0.f1.f0".to_string();
        let perm = crate::testing_utils::TestingUtils::get_permutation_from_operation_string(
            op_string, str_to_gen,
        );
        let expected_perm = crate::permutation::Permutation::parse_permutation_from_cycle(
            "(1,19,24,10)(2,17,23,12)(3,22)(4,21)(5,8)(6,7)(9,20)(11,18)(13,14,16,15)",
            24,
        );
        if perm != expected_perm {
            println!("perm: \t\t {:?}", perm);
            println!("expected_perm: \t {:?}", expected_perm);
        }
        assert!(perm == expected_perm);
    }

    #[test]
    fn test_get_sol_string_from_op_string() {
        let init_string: String = "D;E;D;A;E;B;A;B;C;A;C;A;D;C;D;F;F;F;E;E;B;F;B;C".to_string();
        let expected_string: String = "E;F;F;B;B;A;B;E;E;D;F;E;C;F;D;D;B;C;C;C;A;D;A;A".to_string();
        let op_string: String = "f0.f1.f0".to_string();
        let puzzle_type = crate::puzzle::PuzzleType::CUBE(2);
        let num_wildcards = 0;
        crate::testing_utils::TestingUtils::assert_applying_sol_string_to_initial_string_results_in_target(
            init_string, expected_string, op_string, puzzle_type, num_wildcards,
        );
    }
}
