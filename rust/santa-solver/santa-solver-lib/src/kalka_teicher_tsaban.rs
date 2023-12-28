use crate::groups::PermutationGroupIterator;
use crate::permutation::{Permutation, PermutationIndex, PermutationInfo, PermutationPath};
use crate::testing_utils::TestingUtils;
use log::{debug, info, warn};
use std::collections::{HashMap, HashSet};

fn to_2_cycle(p: &PermutationInfo) -> Vec<Vec<usize>> {
    let cycles = &p.cycles; // disjoint cycles of arbitrary length
                            // Express p as a product of disjoint 2-cycles and fixed points
    let mut result = Vec::new();
    for cycle in cycles {
        if cycle.len() == 1 {
            continue;
        } else if cycle.len() == 2 {
            result.push(cycle.clone());
        } else {
            for i in 1..cycle.len() {
                // TODO: Find all possible 2-cycle decompositions
                result.push(vec![cycle[0], cycle[i]]);
            }
        }
    }
    result
}

fn to_3_cycle(p: &PermutationInfo) -> Vec<Vec<usize>> {
    if !p.signum {
        panic!("Permutation must be even");
    }
    let _cycles = &p.cycles; // disjoint cycles of arbitrary length
                             // Express p as a product of disjoint 3-cycles and fixed points
                             // TODO: implement for better results
    to_2_cycle(p)
}

pub fn find_c_cycle(
    gen_to_str: &HashMap<Permutation, PermutationIndex>,
    c: usize,
    n: usize,
) -> Option<(PermutationPath, Permutation)> {
    let generator = PermutationGroupIterator::new(&gen_to_str);
    let mut i = 0;
    for (mut tau_path, tau) in generator {
        i += 1;
        if i % 1000 == 0 {
            debug!("Generators tried: {:?}", i);
        }
        let mut tau_pow = Permutation::identity(tau.len());
        'inner: for m in 1..=n {
            // Check whether tau.pow(m) is a c-cycle
            tau_pow = tau_pow.compose(&tau);
            let tau_cycles = tau_pow.compute_info().cycles;
            let mut found = false;
            for cycle in tau_cycles {
                if cycle.len() > 1 {
                    if cycle.len() == c {
                        if found {
                            continue 'inner;
                        } else {
                            found = true;
                        }
                    } else {
                        continue 'inner;
                    }
                }
            }
            if found {
                tau_path.pow(m);
                return Some((tau_path, tau_pow));
            }
            if i >= 100000 {
                warn!("Aborting after trying {} generators", i);
                return None;
            }
        }
    }
    None
}

pub fn generate_transpositions(
    gen_to_str: &HashMap<Permutation, PermutationIndex>,
    mu: &Permutation,
    mu_path: &PermutationPath,
    n: usize,
) -> HashMap<Permutation, PermutationPath> {
    let mut a_0: HashMap<Permutation, PermutationPath> = HashMap::new(); // A_{l-1}, previous iteration
    let mut a_l: HashMap<Permutation, PermutationPath> = HashMap::new(); // A_l, current iteration
    let mut a_union: HashMap<Permutation, PermutationPath> = HashMap::new(); // a_0 union A_1 union ... union A_l
    a_0.insert(mu.clone(), mu_path.clone());
    let generators = &gen_to_str
        .keys()
        .map(|x| x.clone())
        .collect::<Vec<Permutation>>();
    loop {
        a_l.clear();
        for gen in generators {
            let s_i = gen;
            let s_i_inv = &s_i.inverse();
            let s_i_path = gen_to_str.get(s_i).unwrap();
            let s_i_inv_path = gen_to_str.get(s_i_inv).unwrap();
            for (a, a_path) in &a_0 {
                // calculate s_i^eps * a * s_i^-eps and check membership
                let perm_eps_pos = s_i_inv.compose(&a.compose(s_i));
                let perm_eps_neg = s_i.compose(&a.compose(s_i_inv));

                // A_union = (a_0 union a_1 union ... union A_{l-1}) at this point
                if !a_union.contains_key(&perm_eps_pos) && !a_l.contains_key(&perm_eps_pos) {
                    // Is the a_l check necessary?
                    let mut al_path = PermutationPath::default();
                    al_path.push(*s_i_inv_path);
                    al_path.merge(a_path);
                    al_path.push(*s_i_path);
                    a_l.insert(perm_eps_pos, al_path);
                }
                if !a_union.contains_key(&perm_eps_neg) && !a_l.contains_key(&perm_eps_neg) {
                    let mut al_path = PermutationPath::default();
                    al_path.push(*s_i_path);
                    al_path.merge(a_path);
                    al_path.push(*s_i_inv_path);
                    a_l.insert(perm_eps_neg, al_path);
                }
            }
        }
        // check if we could find elements in next iteration
        if a_l.is_empty() {
            return a_union;
        }
        // add A_{l} to A_union by extending A_union
        // reassign s.t. we can reach A_{l} in next iteration A_{l+1}
        // Move items out of a_l into a_union
        a_union.extend(a_l.clone());
        std::mem::swap(&mut a_0, &mut a_l);
        if a_union.len() > n {
            info!("Aborting after finding {} elements", a_union.len());
            return a_union;
        }
    }
}

pub fn factorize(
    gen_to_idx: &HashMap<Permutation, PermutationIndex>,
    gen_to_str: Vec<String>,
    target: &Permutation,
) -> Option<String> {
    let permutation_info = target.compute_info();
    let generators = &gen_to_idx
        .keys()
        .map(|x| x.clone())
        .collect::<Vec<Permutation>>();
    debug!("generators: {:?}", generators);
    let n = target.len();
    let c = match permutation_info.signum {
        false => 2,
        true => 3,
    };
    if c != 2 {
        panic!("3-cycles are not implemented yet.");
    }

    // Step 1: Find a short c-cycle in the group generated by generators
    let (mu_path, mu) = match find_c_cycle(&gen_to_idx, c, n) {
        Some((mu_path, mu)) => (mu_path, mu),
        None => {
            eprintln!("No short c-cycle found");
            return None;
        }
    };
    debug!(
        "After step 1, we have mu : {:?} and mu_path: {:?}",
        mu, mu_path
    );

    // disjoint cycles of length c
    let c_group = match permutation_info.signum {
        false => to_2_cycle(&permutation_info),
        true => to_3_cycle(&permutation_info),
    };

    // convert C to a set of permutations
    let mut c_set = HashSet::new();
    for cycle in c_group {
        c_set.insert(Permutation::from_cycles_fixed_per_size(&vec![cycle], n));
    }

    // Step 2: Find short expressions for additional c-cycles
    let mut a_0: HashMap<Permutation, PermutationPath> = HashMap::new(); // A_{l-1}, previous iteration
    let mut a_l: HashMap<Permutation, PermutationPath> = HashMap::new(); // A_l, current iteration
    let mut a_union: HashMap<Permutation, PermutationPath> = HashMap::new(); // a_0 union A_1 union ... union A_l
    a_0.insert(mu, mu_path);
    loop {
        a_l.clear();
        for gen in generators {
            let s_i = gen;
            let s_i_inv = &s_i.inverse();
            let s_i_path = gen_to_idx.get(s_i).unwrap();
            let s_i_inv_path = gen_to_idx.get(s_i_inv).unwrap();
            for (a, a_path) in &a_0 {
                // calculate s_i^eps * a * s_i^-eps and check membership
                let perm_eps_pos = s_i_inv.compose(&a.compose(s_i));
                let perm_eps_neg = s_i.compose(&a.compose(s_i_inv));

                // A_union = (a_0 union a_1 union ... union A_{l-1}) at this point
                if !a_union.contains_key(&perm_eps_pos) && !a_l.contains_key(&perm_eps_pos) {
                    // Is the a_l check necessary?
                    let mut al_path = PermutationPath::default();
                    al_path.push(*s_i_inv_path);
                    al_path.merge(a_path);
                    al_path.push(*s_i_path);
                    a_l.insert(perm_eps_pos, al_path);
                }
                if !a_union.contains_key(&perm_eps_neg) && !a_l.contains_key(&perm_eps_neg) {
                    let mut al_path = PermutationPath::default();
                    al_path.push(*s_i_path);
                    al_path.merge(a_path);
                    al_path.push(*s_i_inv_path);
                    a_l.insert(perm_eps_neg, al_path);
                }
            }
        }
        // check if we could find elements in next iteration
        if a_l.is_empty() {
            warn!("Error: A_l is empty");
            debug!("a_union: {:?}", a_union);
            debug!("a_union length: {:?}", a_union.len());
            debug!("c_set length: {:?}", c_set.len());
            // Print the number of elements in c_set that are not in a_union
            let mut missing = 0;
            for c in &c_set {
                if !a_union.contains_key(c) {
                    missing += 1;
                }
            }
            debug!("missing: {:?}", missing);
            return None;
        }
        // add A_{l} to A_union by extending A_union
        // reassign s.t. we can reach A_{l} in next iteration A_{l+1}
        // Move items out of a_l into a_union
        a_union.extend(a_l.clone());
        std::mem::swap(&mut a_0, &mut a_l);
        // Count the elements from c_set missing in a_union
        let mut missing = 0;
        for c in &c_set {
            if !a_union.contains_key(c) {
                missing += 1;
            }
        }
        if missing == 0 {
            // We have found a short expression for all c-cycles
            // Return a_union
            let result = a_union.get(target).unwrap();
            return Some(result.to_string(&gen_to_str));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::testing_utils::TestingUtils;

    use super::*;

    #[test]
    fn test_generate_transposition() {
        let gen_to_index = TestingUtils::get_generator_to_perm_index_map_s_n(5);
        let (mu_path, mu) = find_c_cycle(&gen_to_index, 2, 5).unwrap();
        let result = generate_transpositions(&gen_to_index, &mu, &mu_path, 10);
        for (perm, path) in result {
            TestingUtils::assert_index_path_equals_permutation_using_hashmap(
                &path.arr,
                &perm,
                &gen_to_index,
            );
        }
    }

    #[test]
    fn test_find_c_cycle_s_5() {
        let gen_to_index = TestingUtils::get_generator_to_perm_index_map_s_n(5);
        let index_to_gen = TestingUtils::get_index_to_perm_vec_s_n(5);
        let test_ranges = vec![2, 3, 4];
        for c in test_ranges {
            let result = find_c_cycle(&gen_to_index, c, 5);
            assert!(result.is_some());
            let (path, result) = result.unwrap();
            TestingUtils::assert_index_path_equals_permutation(&path.arr, &result, &index_to_gen);
            // check that result is a c-cycle (return value is non deterministic)
            let resultinfo = result.compute_info();
            debug!("result: {:?}", resultinfo);
            TestingUtils::assert_cycle_list_is_c_cycle(resultinfo.cycles, c)
        }
    }

    #[test]
    fn test_find_2_cycle() {
        let gen1 = Permutation::parse_permutation_from_cycle("(1,2)", 3);
        let gen2 = Permutation::parse_permutation_from_cycle("(2,3)", 3);
        let mut gen_to_str: HashMap<Permutation, usize> = HashMap::new();
        gen_to_str.insert(gen1.clone(), 0);
        gen_to_str.insert(gen2.clone(), 1);
        let result = find_c_cycle(&gen_to_str, 2, 3);
        assert!(result.is_some());
        let (_path, result) = result.unwrap();
        // check that result is a 2-cycle (return value is non deterministic)
        let resultinfo = result.compute_info();
        debug!("result: {:?}", resultinfo);
        for cycle in result.compute_info().cycles {
            if cycle.len() == 1 {
                continue;
            }
            assert_eq!(cycle.len(), 2);
        }
    }
    //
    // #[test]
    // fn factorize_simple() {
    //     let gen1 = Permutation::parse_permutation_from_cycle("(1,2)", 3);
    //     let gen2 = Permutation::parse_permutation_from_cycle("(2,3)", 3);
    //     let gen3 = gen1.inverse();
    //     let gen4 = gen2.inverse();
    //     let mut gen_to_str: HashMap<Permutation, usize> = HashMap::new();
    //     let mut str_to_gen: HashMap<String, Permutation> = HashMap::new();
    //     gen_to_str.insert(gen1.clone(), 0);
    //     gen_to_str.insert(gen2.clone(), 1);
    //     gen_to_str.insert(gen3.clone(), 2);
    //     gen_to_str.insert(gen4.clone(), 3);
    //     str_to_gen.insert("gen1".to_string(), gen1.clone());
    //     str_to_gen.insert("gen2".to_string(), gen2.clone());
    //     str_to_gen.insert("-gen1".to_string(), gen3.clone());
    //     str_to_gen.insert("-gen2".to_string(), gen4.clone());
    //     let target = Permutation::new(vec![1, 3, 2]);
    //
    //     let gen_to_idx = gen_to_str
    //         .iter()
    //         .enumerate()
    //         .map(|(i, (p, _))| (p.clone(), i))
    //         .collect::<HashMap<Permutation, PermutationIndex>>();
    //     let index_to_gen_name = vec!["gen1".to_string(), "gen2".to_string()];
    //     let result = factorize(&gen_to_idx, index_to_gen_name, &target).unwrap();
    //     debug!("result: {:?}", result);
    //     // check if result is correct (factorization results in target permutation)
    //     let mut result_perm = Permutation::identity(3);
    //     for move_name in result.split(".") {
    //         println!("move_name: {:?}", move_name);
    //         let move_perm = str_to_gen.get(move_name).unwrap();
    //         result_perm = move_perm.compose(&result_perm);
    //     }
    //     debug!("result: {:?}", result_perm);
    //     debug!("target: {:?}", target);
    //     assert_eq!(result_perm, target);
    // }

    // #[test]
    // fn factorize_larger() {
    //     let gen1 = Permutation::parse_permutation_from_cycle("(1,2)", 10);
    //     let gen2 = Permutation::parse_permutation_from_cycle("(2,3)", 10);
    //     let gen3 = Permutation::parse_permutation_from_cycle("(3,4)", 10);
    //     let gen4 = Permutation::parse_permutation_from_cycle("(4,5)", 10);
    //     let gen5 = Permutation::parse_permutation_from_cycle("(5,6)", 10);
    //     let gen6 = Permutation::parse_permutation_from_cycle("(6,7)", 10);
    //     let gen7 = Permutation::parse_permutation_from_cycle("(7,8)", 10);
    //     let gen8 = Permutation::parse_permutation_from_cycle("(8,9)", 10);
    //     let gen9 = Permutation::parse_permutation_from_cycle("(9,10)", 10);
    //     let gen1_inv = gen1.inverse();
    //     let gen2_inv = gen2.inverse();
    //     let gen3_inv = gen3.inverse();
    //     let gen4_inv = gen4.inverse();
    //     let gen5_inv = gen5.inverse();
    //     let gen6_inv = gen6.inverse();
    //     let gen7_inv = gen7.inverse();
    //     let gen8_inv = gen8.inverse();
    //     let gen9_inv = gen9.inverse();
    //
    //     let mut gen_to_str: HashMap<Permutation, String> = HashMap::new();
    //     let mut str_to_gen: HashMap<String, Permutation> = HashMap::new();
    //     str_to_gen.insert("gen1".to_string(), gen1.clone());
    //     str_to_gen.insert("gen2".to_string(), gen2.clone());
    //     str_to_gen.insert("gen3".to_string(), gen3.clone());
    //     str_to_gen.insert("gen4".to_string(), gen4.clone());
    //     str_to_gen.insert("gen5".to_string(), gen5.clone());
    //     str_to_gen.insert("gen6".to_string(), gen6.clone());
    //     str_to_gen.insert("gen7".to_string(), gen7.clone());
    //     str_to_gen.insert("gen8".to_string(), gen8.clone());
    //     str_to_gen.insert("gen9".to_string(), gen9.clone());
    //
    //     str_to_gen.insert("-gen1".to_string(), gen1_inv.clone());
    //     str_to_gen.insert("-gen2".to_string(), gen2_inv.clone());
    //     str_to_gen.insert("-gen3".to_string(), gen3_inv.clone());
    //     str_to_gen.insert("-gen4".to_string(), gen4_inv.clone());
    //     str_to_gen.insert("-gen5".to_string(), gen5_inv.clone());
    //     str_to_gen.insert("-gen6".to_string(), gen6_inv.clone());
    //     str_to_gen.insert("-gen7".to_string(), gen7_inv.clone());
    //     str_to_gen.insert("-gen8".to_string(), gen8_inv.clone());
    //     str_to_gen.insert("-gen9".to_string(), gen9_inv.clone());
    //     let index_to_gen_name = vec![
    //         "gen1".to_string(),
    //         "gen2".to_string(),
    //         "gen3".to_string(),
    //         "gen4".to_string(),
    //         "gen5".to_string(),
    //         "gen6".to_string(),
    //         "gen7".to_string(),
    //         "gen8".to_string(),
    //         "gen9".to_string(),
    //     ];
    //     for gen in &index_to_gen_name {
    //         let perm = str_to_gen.get(gen).unwrap();
    //         gen_to_str.insert(perm.clone(), gen.clone());
    //         gen_to_str.insert(perm.inverse(), format!("-{}", gen));
    //     }
    //
    //     let target = Permutation::new(vec![1, 2, 3, 4, 5, 6, 7, 9, 8, 10]);
    //
    //     let gen_to_idx = gen_to_str
    //         .iter()
    //         .enumerate()
    //         .map(|(i, (p, _))| (p.clone(), i))
    //         .collect::<HashMap<Permutation, PermutationIndex>>();
    //
    //     let result = factorize(&gen_to_idx, index_to_gen_name, &target).unwrap();
    //     // check if result is correct (factorization results in target permutation)
    //     println!("result: {:?}", result);
    //     let mut result_perm = Permutation::identity(10);
    //     for move_name in result.split(".") {
    //         let move_perm = str_to_gen.get(move_name).unwrap();
    //         result_perm = move_perm.compose(&result_perm);
    //     }
    //     let mut expected_perm = Permutation::identity(10);
    //     expected_perm = gen8.compose(&expected_perm);
    //     debug!("result: {:?}", result_perm);
    //     debug!("target: {:?}", target);
    //     assert_eq!(result_perm, expected_perm);
    // }
}
