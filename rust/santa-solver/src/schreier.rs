use crate::permutation::Permutation;
use crate::{groups, permutation::PermutationPath};
use rand;
use rust_schreier::perm::Perm;
use rust_schreier::schreier;
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq)]
pub struct SchreierSims {
    pub vector: Vec<Option<(usize, usize)>>,
    k: usize,
}

impl SchreierSims {
    pub fn find_base(gens: Vec<Permutation>) -> Vec<usize> {
        let n = gens[0].p.len();
        let mut base: Vec<usize> = vec![];
        let mut rnd = rand::thread_rng();
        let mut gen: Vec<Perm> = vec![];
        for perm in gens {
            let new_perm = Perm::new(perm.p.iter().map(|x| *x - 1).collect());
            gen.push(new_perm);
        }
        let (betas, _) = schreier::incrementally_build_bsgs(n, &base, &gen, &mut rnd);
        for elm in &betas {
            base.push(elm.0);
        }
        base
    }

    /*
     * Returns a coset traversal of representative elements given a function that tests if an element is a subgroup member.
     * The function is_subgroup_member should return true if the element is a subgroup member and false otherwise.
     */
    pub fn get_coset_traversal(
        gens: &HashMap<Permutation, PermutationPath>,
        is_subgroup_member: &fn(&Permutation) -> bool,
    ) -> Vec<(Permutation, PermutationPath)> {
        let mut coset_traversal: Vec<(Permutation, PermutationPath)> = vec![];
        let gens_depr: HashMap<Permutation, usize> = gens
            .iter()
            .map(|(perm, path)| (perm.clone(), path.arr[0]))
            .collect();
        let group_generator = groups::PermutationGroupIterator::new(&gens_depr);
        for (perm_path, perm) in group_generator {
            let found_identical_coset = coset_traversal.iter().any(|(repr, path)| {
                let group_elm = perm.compose(&repr.inverse());
                return is_subgroup_member(&group_elm);
            });
            if !found_identical_coset {
                coset_traversal.push((perm, perm_path));
            }
        }
        return coset_traversal;
    }

    /*
     * Returns a set of subgroup generators for a subgroup given a coset traversal and a function that tests if an element is a subgroup member.
     * The function is_subgroup_member should return true if the element is a subgroup member and false otherwise.
     * The coset traversal should be a list of coset representatives.
     */
    pub fn get_subgroup_gens_from_coset_traversal(
        coset_traversal: &Vec<(Permutation, PermutationPath)>,
        current_gens: &HashMap<Permutation, PermutationPath>,
        is_subgroup_member: &fn(&Permutation) -> bool,
    ) -> HashMap<Permutation, PermutationPath> {
        let mut subgroup_gens: HashMap<Permutation, PermutationPath> = HashMap::new();
        for (t, t_path) in coset_traversal {
            for s in current_gens.keys() {
                let perm = s.compose(&t);
                let mut perm_representative: Option<Permutation> = None;
                // find coset representative for perm
                for (t2, t2_path) in coset_traversal {
                    let perm2 = t2.compose(&perm.inverse());
                    if is_subgroup_member(&perm2) {
                        perm_representative = Some(t2.clone());
                        break;
                    }
                }
                if perm_representative.is_none() {
                    println!("error!");
                    println!("t: {:?}, s: {:?}, perm: {:?}", t, s, perm);
                    panic!("Each element should have a coset representative!");
                }
                let perm_representative = perm_representative.unwrap();
                let perm = perm.compose(&perm_representative.inverse());
                println!("perm: {:?}", perm);
                if !subgroup_gens.contains_key(&perm.inverse()) {
                    subgroup_gens.insert(perm, PermutationPath::new(vec![0]));
                }
            }
        }
        return subgroup_gens;
    }

    /*
     * Given a list of subgroups, that are specified by a list of membership testing functions,
     * returns a list of generators that generate the intersection of all subgroups.
     */
    pub fn relaxed_schreier_sims(
        initial_gens: HashMap<Permutation, PermutationPath>,
        subgroup_testing_functions: Vec<fn(&Permutation) -> bool>,
    ) -> HashMap<Permutation, PermutationPath> {
        let mut current_gens = initial_gens;
        for subgroup_testing_function in &subgroup_testing_functions {
            let coset_traversal =
                SchreierSims::get_coset_traversal(&current_gens, subgroup_testing_function);
            let subgroup_gens_new = SchreierSims::get_subgroup_gens_from_coset_traversal(
                &coset_traversal,
                &current_gens,
                subgroup_testing_function,
            );
            current_gens = subgroup_gens_new;
        }
        return current_gens;
    }

    pub fn test_indices_interchangeable(
        perm: &Permutation,
        interchangeable_indices: &HashSet<usize>,
    ) -> bool {
        let perm = &perm.p;
        for index in interchangeable_indices {
            if !interchangeable_indices.contains(&(perm[*index] - 1)) {
                return false;
            }
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;
    use crate::permutation::Permutation;

    #[test]
    fn test_find_base() {
        let perm1 = Permutation::parse_permutation_from_cycle("(1,5,7)(2,6,8)", 8);
        let perm2 = Permutation::parse_permutation_from_cycle("(1,5)(3,4,8,2)", 8);
        let gens = vec![perm1, perm2];
        let base = SchreierSims::find_base(gens);
        assert_eq!(base.len(), 5);
    }

    #[test]
    fn test_find_base_rubik_small() {
        let perm1 =
            Permutation::parse_permutation_from_cycle("(9,10,12,11)(3,13,22,8)(4,15,21,6)", 24);
        let perm2 =
            Permutation::parse_permutation_from_cycle("(17,18,20,19)(1,7,24,14)(2,5,23,16)", 24);
        let perm3 =
            Permutation::parse_permutation_from_cycle("(1,2,4,3)(9,5,17,13)(10,6,18,14)", 24);
        let perm4 =
            Permutation::parse_permutation_from_cycle("(21,22,24,23)(11,15,19,7)(12,16,20,8)", 24);
        let perm5 =
            Permutation::parse_permutation_from_cycle("(5,6,8,7)(9,21,20,1)(11,23,18,3)", 24);
        let perm6 =
            Permutation::parse_permutation_from_cycle("(13,14,16,15)(10,2,19,22)(12,4,17,24)", 24);
        let gens = vec![perm1, perm2, perm3, perm4, perm5, perm6];
        let base = SchreierSims::find_base(gens);
        assert_eq!(base.len(), 7);
    }

    #[test]
    fn test_coset_traversal() {
        let g1 = Permutation::parse_permutation_from_cycle("(1,2)", 3);
        let g2 = Permutation::parse_permutation_from_cycle("(1,2,3)", 3);
        let is_in_subgroup: fn(&Permutation) -> bool = |perm| {
            let sub_elm1 = Permutation::parse_permutation_from_cycle("(1,2,3)", 3);
            let sub_elm2 = Permutation::parse_permutation_from_cycle("(1,3,2)", 3);
            let sub_elm3 = Permutation::parse_permutation_from_cycle("(1)", 3);
            let subgroup: Vec<Permutation> = vec![sub_elm1, sub_elm2, sub_elm3];
            return subgroup.iter().any(|sub_elm| sub_elm == perm);
        };

        let subgroup_gens: HashMap<Permutation, PermutationPath> = vec![
            (g1, PermutationPath::new(vec![0])),
            (g2, PermutationPath::new(vec![1])),
        ]
        .into_iter()
        .collect();

        let coset_traversal = SchreierSims::get_coset_traversal(&subgroup_gens, &is_in_subgroup);
        assert_eq!(coset_traversal.len(), 2);
    }

    #[test]
    fn test_coset_traversal_fix_certain() {
        let g1 = Permutation::parse_permutation_from_cycle("(1,2)", 3);
        let g2 = Permutation::parse_permutation_from_cycle("(1,2,3)", 3);
        let indices_interchangeable: fn(&Permutation) -> bool = |perm| {
            let interchangeable_indices: HashSet<usize> = vec![0, 1].into_iter().collect();
            let perm = &perm.p;
            for index in &interchangeable_indices {
                if !interchangeable_indices.contains(&(perm[*index] - 1)) {
                    return false;
                }
            }
            return true;
        };
        let subgroup_gens: HashMap<Permutation, PermutationPath> = vec![
            (g1, PermutationPath::new(vec![0])),
            (g2, PermutationPath::new(vec![1])),
        ]
        .into_iter()
        .collect();
        let coset_traversal =
            SchreierSims::get_coset_traversal(&subgroup_gens, &indices_interchangeable);
        // check that every index in the generated coset traversal is interchangeable
        assert_eq!(coset_traversal.len(), 3);
    }

    #[test]
    fn test_get_subgroup_gens_from_coset_traversal_small() {
        let g1 = Permutation::parse_permutation_from_cycle("(1,2)", 3);
        let g2 = Permutation::parse_permutation_from_cycle("(1,2,3)", 3);
        let is_in_subgroup: fn(&Permutation) -> bool = |perm| {
            let sub_elm1 = Permutation::parse_permutation_from_cycle("(1,2,3)", 3);
            let sub_elm2 = Permutation::parse_permutation_from_cycle("(1,3,2)", 3);
            let sub_elm3 = Permutation::parse_permutation_from_cycle("(1)", 3);
            let subgroup: Vec<Permutation> = vec![sub_elm1, sub_elm2, sub_elm3];
            return subgroup.iter().any(|sub_elm| sub_elm.p == perm.p);
        };

        let subgroup_gens: HashMap<Permutation, PermutationPath> = vec![
            (g1, PermutationPath::new(vec![0])),
            (g2, PermutationPath::new(vec![1])),
        ]
        .into_iter()
        .collect();

        let coset_traversal = SchreierSims::get_coset_traversal(&subgroup_gens, &is_in_subgroup);
        println!("coset traversal: {:?}", coset_traversal);
        assert_eq!(coset_traversal.len(), 2);
        let subgroup_gens = SchreierSims::get_subgroup_gens_from_coset_traversal(
            &coset_traversal,
            &subgroup_gens,
            &is_in_subgroup,
        );
        for subgroup_gen in &subgroup_gens {
            println!("subgroup gen: {:?}", subgroup_gen);
            assert!(is_in_subgroup(subgroup_gen.0));
        }
        assert_eq!(subgroup_gens.len(), 2);
    }

    #[test]
    fn test_get_subgroup_gens_from_coset_traversal() {
        let g1 = Permutation::parse_permutation_from_cycle("(1,2)", 3);
        let g2 = Permutation::parse_permutation_from_cycle("(1,2,3)", 3);
        let subgroup_gens: HashMap<Permutation, PermutationPath> = vec![
            (g1, PermutationPath::new(vec![0])),
            (g2, PermutationPath::new(vec![1])),
        ]
        .into_iter()
        .collect();
        let indices_interchangeable: fn(&Permutation) -> bool = |perm| {
            let interchangeable_indices: HashSet<usize> = vec![0, 1].into_iter().collect();
            let perm = &perm.p;
            for index in &interchangeable_indices {
                if !interchangeable_indices.contains(&(perm[*index] - 1)) {
                    return false;
                }
            }
            return true;
        };
        let coset_traversal =
            SchreierSims::get_coset_traversal(&subgroup_gens, &indices_interchangeable);
        assert_eq!(coset_traversal.len(), 3);
        println!("coset traversal: {:?}", coset_traversal);
        let subgroup_gens = SchreierSims::get_subgroup_gens_from_coset_traversal(
            &coset_traversal,
            &subgroup_gens,
            &indices_interchangeable,
        );
        for subgroup_gen in &subgroup_gens {
            println!("subgroup gen: {:?}", subgroup_gen);
            assert!(indices_interchangeable(subgroup_gen.0));
        }
        assert_eq!(subgroup_gens.len(), 2);
    }

    #[test]
    fn test_relaxed_minkwitz() {
        // Imagine we have following initial string:    aabbcc
        // We number the initial string like this:      012345
        // We want to find a permutation that transforms this string into abcabc
        // For this, we get are given S_6, and the union of the subgroups that stabilize (0,1), (2,3), (4,5) by keeping them interchangeable
        let g1 = Permutation::parse_permutation_from_cycle("(1,2)", 6);
        let g2 = Permutation::parse_permutation_from_cycle("(2,3)", 6);
        let g3 = Permutation::parse_permutation_from_cycle("(3,4)", 6);
        let g4 = Permutation::parse_permutation_from_cycle("(4,5)", 6);

        let group_gens: HashMap<Permutation, PermutationPath> = vec![
            (g1, PermutationPath::new(vec![0])),
            (g2, PermutationPath::new(vec![1])),
            (g3, PermutationPath::new(vec![2])),
            (g4, PermutationPath::new(vec![3])),
        ]
        .into_iter()
        .collect();
        let membership_test_1: fn(&Permutation) -> bool = |perm| {
            let stab_ind_1 = vec![0, 1].into_iter().collect();
            return SchreierSims::test_indices_interchangeable(perm, &stab_ind_1);
        };
        let membership_test_2: fn(&Permutation) -> bool = |perm| {
            let stab_ind_2 = vec![2, 3].into_iter().collect();
            return SchreierSims::test_indices_interchangeable(perm, &stab_ind_2);
        };
        let membership_test_3: fn(&Permutation) -> bool = |perm| {
            let stab_ind_3 = vec![4, 5].into_iter().collect();
            return SchreierSims::test_indices_interchangeable(perm, &stab_ind_3);
        };

        let subgroup_testing_functions: Vec<fn(&Permutation) -> bool> =
            vec![membership_test_1, membership_test_2, membership_test_3];

        let subgroup_gens =
            SchreierSims::relaxed_schreier_sims(group_gens, subgroup_testing_functions);
        for subgroup_gen in &subgroup_gens {
            println!("subgroup gen: {:?}", subgroup_gen);
            assert!(membership_test_1(subgroup_gen.0));
            assert!(membership_test_2(subgroup_gen.0));
            assert!(membership_test_3(subgroup_gen.0));
        }

        assert_eq!(subgroup_gens.len(), 4);
    }
}
