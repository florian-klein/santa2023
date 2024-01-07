use crate::groups;
use crate::minkwitz::PermAndWord;
use crate::permutation::Permutation;
use log::info;
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

    pub fn get_schreier_vector(
        k_indices: &HashSet<usize>,
        generators: &HashSet<&PermAndWord>,
    ) -> Vec<i32> {
        let perm_length = generators.iter().next().unwrap().perm.p.len();
        // array of length perm_length
        let mut schreier_vector: Vec<i32> = vec![-1; perm_length];
        let mut number_of_changes = 1;
        // set all elements in k_indices to 0
        for index in k_indices {
            schreier_vector[*index] = 0;
        }
        while number_of_changes > 0 {
            number_of_changes = 0;
            for i in 0..perm_length {
                if schreier_vector[i] == -1 {
                    continue;
                }
                // For each r = 1, 2, …, m, we set g = gr:
                // set j := g(i); if v[j] = -1, then set v[j] = 2r-1;
                // set j := g-1(i): if v[j] = -1, then set v[j] = 2r.
                for gen in generators {
                    let j = gen.perm.p[i] - 1;
                    let r = gen.word[0];
                    if schreier_vector[j] == -1 {
                        schreier_vector[j] = (2 * r + 1) as i32;
                        number_of_changes += 1;
                    }
                }
            }
        }

        return schreier_vector;
    }

    pub fn get_coset_traversal_schreier(
        index_to_perm_and_word: &Vec<&PermAndWord>,
        schreier_vector: &Vec<i32>,
    ) -> Vec<PermAndWord> {
        let mut coset_traversal: Vec<PermAndWord> = vec![];
        let mut k_index_seen = false;
        for i in 0..schreier_vector.len() {
            if schreier_vector[i] == -1 {
                continue;
            }
            if schreier_vector[i] == 0 {
                if !k_index_seen {
                    let identity =
                        PermAndWord::new(Permutation::identity(schreier_vector.len()), vec![]);
                    coset_traversal.push(identity);
                    k_index_seen = true;
                    continue;
                }
                continue;
            }
            let mut gen_indices: Vec<i32> = vec![];
            let mut j = i;
            while schreier_vector[j] != 0 {
                let r = schreier_vector[j] / 2;
                let perm_used = &index_to_perm_and_word[r as usize];
                gen_indices.push(r + 1);
                j = perm_used.perm.inverse().p[j] - 1;
            }
            // decode coset traversal from indices and add to coset traversal
            let mut coset_traversal_elm =
                PermAndWord::new(Permutation::identity(schreier_vector.len()), vec![]);
            for index in gen_indices {
                let next_elm = &index_to_perm_and_word[index as usize - 1];
                coset_traversal_elm = next_elm.compose(&coset_traversal_elm);
            }
            coset_traversal.push(coset_traversal_elm);
        }

        return coset_traversal;
    }

    /*
     * Returns a coset traversal of representative elements given a function that tests if an element is a subgroup member.
     * The function is_subgroup_member should return true if the element is a subgroup member and false otherwise.
     */
    pub fn get_coset_traversal(
        gens: &HashSet<PermAndWord>,
        valid_indices: &HashSet<usize>,
    ) -> Vec<PermAndWord> {
        let mut coset_traversal: Vec<PermAndWord> = vec![];
        let group_generator = groups::PermutationGroupPermAndWordIterator::new(&gens);
        let mut counter = 0;
        for perm_and_word in group_generator {
            let found_identical_coset = coset_traversal.iter().any(|repr| {
                let group_elm = perm_and_word.compose(&repr.get_inverse());
                return Self::test_indices_interchangeable(&group_elm.perm, valid_indices);
            });
            if !found_identical_coset {
                coset_traversal.push(perm_and_word);
            }
            // there wouldn't be more than 1000 cosets for a group
            if counter > 10000 {
                break;
            }
            counter += 1;
        }
        return coset_traversal;
    }

    pub fn get_subgroup_gens_from_coset_traversal_schreier(
        coset_traversal: &Vec<PermAndWord>,
        current_gens: &HashSet<&PermAndWord>,
        valid_indices: &HashSet<usize>,
    ) -> HashSet<PermAndWord> {
        let mut subgroup_gens: HashSet<PermAndWord> = HashSet::new();
        for t in coset_traversal {
            for s in current_gens {
                let perm = s.compose(&t);
                let mut perm_representative: Option<PermAndWord> = None;
                // find coset representative for perm
                for t2 in coset_traversal {
                    let perm2 = t2.compose(&perm.get_inverse());
                    println!("perm2: {:?}", perm2);
                    if Self::test_indices_interchangeable(&perm2.perm, valid_indices) {
                        perm_representative = Some(t2.clone());
                        break;
                    }
                }
                if perm_representative.is_none() {
                    println!("error!");
                    println!("perm: {:?}", perm);
                    panic!("Each element should have a coset representative!");
                }
                let perm_representative = perm_representative.unwrap();
                let perm = perm.compose(&perm_representative.get_inverse());
                println!("perm: {:?}", perm);
                if !subgroup_gens.contains(&perm.get_inverse()) {
                    subgroup_gens.insert(perm);
                }
            }
        }
        return subgroup_gens;
    }

    /*
     * Returns a set of subgroup generators for a subgroup given a coset traversal and a function that tests if an element is a subgroup member.
     * The function is_subgroup_member should return true if the element is a subgroup member and false otherwise.
     * The coset traversal should be a list of coset representatives.
     */
    pub fn get_subgroup_gens_from_coset_traversal(
        coset_traversal: &Vec<PermAndWord>,
        current_gens: &HashSet<PermAndWord>,
        valid_indices: &HashSet<usize>,
    ) -> HashSet<PermAndWord> {
        let mut subgroup_gens: HashSet<PermAndWord> = HashSet::new();
        for t in coset_traversal {
            for s in current_gens {
                let perm = s.compose(&t);
                let mut perm_representative: Option<PermAndWord> = None;
                // find coset representative for perm
                for t2 in coset_traversal {
                    let perm2 = t2.compose(&perm.get_inverse());
                    if Self::test_indices_interchangeable(&perm2.perm, valid_indices) {
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
                let perm = perm.compose(&perm_representative.get_inverse());
                if !subgroup_gens.contains(&perm.get_inverse()) {
                    subgroup_gens.insert(perm);
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
        initial_gens: HashSet<PermAndWord>,
        valid_indices: Vec<HashSet<usize>>,
    ) -> HashSet<PermAndWord> {
        let mut current_gens = initial_gens;
        let mut counter = 0;
        for index_group in &valid_indices {
            info!("Starting coset traversal for subgroup {}", counter);
            let coset_traversal = SchreierSims::get_coset_traversal(&current_gens, index_group);
            info!(
                "Found coset traversal for subgroup {} of size {}",
                counter,
                coset_traversal.len()
            );
            let subgroup_gens_new = SchreierSims::get_subgroup_gens_from_coset_traversal(
                &coset_traversal,
                &current_gens,
                index_group,
            );
            info!(
                "Found subgroup gens for subgroup {} of size {}",
                counter,
                subgroup_gens_new.len()
            );
            current_gens = subgroup_gens_new;
            counter += 1;
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

    /*
     * Given two strings, enumerates the solution state, giving each letter a unique number.
     * Then, it enumerates the initial state, assigning each identical letter the incrementing number
     * given in the initial state.
     * Example: solution_state = "a;b;a;b"
     * 1) Enumerate solution state: "a;b;a;b" -> [0,1,2,3]
     * 3) For each color (identical letter) return the set of indices that need to be stabilized.
     */
    pub fn get_stabilizing_color_gens(solution_string: &String) -> Vec<HashSet<usize>> {
        let mut color_to_indices: HashMap<String, HashSet<usize>> = HashMap::new();
        let mut index = 0;
        let sol_string: Vec<String> = solution_string.split(";").map(|x| x.to_string()).collect();
        for c in &sol_string {
            if !color_to_indices.contains_key(c) {
                color_to_indices.insert(c.to_string(), HashSet::new());
            }
            color_to_indices.get_mut(c).unwrap().insert(index);
            index += 1;
        }
        return color_to_indices.values().cloned().collect();
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
    fn test_get_schreier_vector_single() {
        let a = Permutation::parse_permutation_from_cycle("(1,5,3,2)(4,7,9)", 9);
        let b = Permutation::parse_permutation_from_cycle("(1,6,7)", 9);
        let a_perm_word = PermAndWord::new_with_inverse(a.clone(), vec![0], vec![2]);
        let b_perm_word = PermAndWord::new_with_inverse(b.clone(), vec![1], vec![3]);
        let a_perm_word_inv = a_perm_word.get_inverse();
        let b_perm_word_inv = b_perm_word.get_inverse();
        let gen_vec: Vec<&PermAndWord> = vec![
            &a_perm_word,
            &b_perm_word,
            &a_perm_word_inv,
            &b_perm_word_inv,
        ];
        let gens = gen_vec.into_iter().collect();
        let k_indices = vec![0].into_iter().collect();
        let schreier_vector = SchreierSims::get_schreier_vector(&k_indices, &gens);
        assert_eq!(schreier_vector, vec![0, 3, 3, 3, 1, 3, 1, -1, 1]);
    }

    #[test]
    fn test_get_schreier_vector_double() {
        let a = Permutation::parse_permutation_from_cycle("(1,5,3,2)(4,7,9)", 9);
        let b = Permutation::parse_permutation_from_cycle("(1,6)", 9);
        let a_perm_word = PermAndWord::new_with_inverse(a.clone(), vec![0], vec![2]);
        let b_perm_word = PermAndWord::new_with_inverse(b.clone(), vec![1], vec![3]);
        let a_perm_word_inv = a_perm_word.get_inverse();
        let b_perm_word_inv = b_perm_word.get_inverse();
        let gen_vec: Vec<&PermAndWord> = vec![
            &a_perm_word,
            &b_perm_word,
            &a_perm_word_inv,
            &b_perm_word_inv,
        ];
        let gens = gen_vec.into_iter().collect();
        let k_indices = vec![0, 6].into_iter().collect();
        let schreier_vector = SchreierSims::get_schreier_vector(&k_indices, &gens);
        assert_eq!(schreier_vector, vec![0, 3, 3, 3, 1, 3, 0, -1, 1]);
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
    fn test_coset_traversal_large_fix_one() {
        let g1 = Permutation::parse_permutation_from_cycle("(1,5,3,2)(4,7,9)", 9);
        let g2 = Permutation::parse_permutation_from_cycle("(1,6,7)", 9);
        let g1_inv = g1.inverse();
        let g2_inv = g2.inverse();
        // create perm_words for each Permutation
        let g1_perm_word = PermAndWord::new_with_inverse(g1.clone(), vec![0], vec![2]);
        let g2_perm_word = PermAndWord::new_with_inverse(g2.clone(), vec![1], vec![3]);
        let g1_inv_perm_word = PermAndWord::new_with_inverse(g1_inv.clone(), vec![2], vec![0]);
        let g2_inv_perm_word = PermAndWord::new_with_inverse(g2_inv.clone(), vec![3], vec![1]);

        let index_to_perm_and_word = vec![
            &g1_perm_word,
            &g2_perm_word,
            &g1_inv_perm_word,
            &g2_inv_perm_word,
        ];

        let subgroup_gens: HashSet<&PermAndWord> = vec![
            &g1_perm_word,
            &g2_perm_word,
            &g1_inv_perm_word,
            &g2_inv_perm_word,
        ]
        .into_iter()
        .collect();

        let indices_interchangeable: HashSet<usize> = vec![0].into_iter().collect();
        // find schreier vector
        let schreier_vector =
            SchreierSims::get_schreier_vector(&indices_interchangeable, &subgroup_gens);
        // from schreier vector obtain coset traversal
        let coset_traversal =
            SchreierSims::get_coset_traversal_schreier(&index_to_perm_and_word, &schreier_vector);

        for elm in &coset_traversal {
            println!("elm: {:?}", elm);
        }
        assert_eq!(coset_traversal.len(), 8);
    }

    #[test]
    fn test_coset_traversal_fix_certain() {
        let g1 = Permutation::parse_permutation_from_cycle("(1,2)", 3);
        let g2 = Permutation::parse_permutation_from_cycle("(1,2,3)", 3);
        let indices_interchangeable: HashSet<usize> = vec![0, 1].into_iter().collect();
        let subgroup_gens: HashSet<PermAndWord> =
            vec![PermAndWord::new(g1, vec![0]), PermAndWord::new(g2, vec![1])]
                .into_iter()
                .collect();
        let coset_traversal =
            SchreierSims::get_coset_traversal(&subgroup_gens, &indices_interchangeable);
        // check that every index in the generated coset traversal is interchangeable
        println!("coset traversal: {:?}", coset_traversal);
        assert_eq!(coset_traversal.len(), 3);
    }

    #[test]
    fn test_get_subgroup_gens_from_coset_traversal() {
        let g1 = Permutation::parse_permutation_from_cycle("(1,2)", 3);
        let g2 = Permutation::parse_permutation_from_cycle("(1,2,3)", 3);
        let indices_interchangeable: fn(&PermAndWord) -> bool = |perm| {
            let interchangeable_indices: HashSet<usize> = vec![0, 1].into_iter().collect();
            let perm = &perm.perm.p;
            for index in &interchangeable_indices {
                if !interchangeable_indices.contains(&(perm[*index] - 1)) {
                    return false;
                }
            }
            return true;
        };
        let valid_indices = vec![0, 1].into_iter().collect();
        let subgroup_gens: HashSet<PermAndWord> =
            vec![PermAndWord::new(g1, vec![0]), PermAndWord::new(g2, vec![1])]
                .into_iter()
                .collect();
        let coset_traversal = SchreierSims::get_coset_traversal(&subgroup_gens, &valid_indices);
        assert_eq!(coset_traversal.len(), 3);
        println!("coset traversal: {:?}", coset_traversal);
        let subgroup_gens = SchreierSims::get_subgroup_gens_from_coset_traversal(
            &coset_traversal,
            &subgroup_gens,
            &valid_indices,
        );
        for subgroup_gen in &subgroup_gens {
            println!("subgroup gen: {:?}", subgroup_gen);
            assert!(indices_interchangeable(subgroup_gen));
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

        let group_gens: HashSet<PermAndWord> = vec![
            PermAndWord::new(g1, vec![0]),
            PermAndWord::new(g2, vec![2]),
            PermAndWord::new(g3, vec![4]),
            PermAndWord::new(g4, vec![6]),
        ]
        .into_iter()
        .collect();

        let membership_test_1: fn(&PermAndWord) -> bool = |perm| {
            let stab_ind_1 = vec![0, 1].into_iter().collect();
            return SchreierSims::test_indices_interchangeable(&perm.perm, &stab_ind_1);
        };
        let membership_test_2: fn(&PermAndWord) -> bool = |perm| {
            let stab_ind_2 = vec![2, 3].into_iter().collect();
            return SchreierSims::test_indices_interchangeable(&perm.perm, &stab_ind_2);
        };
        let membership_test_3: fn(&PermAndWord) -> bool = |perm| {
            let stab_ind_3 = vec![4, 5].into_iter().collect();
            return SchreierSims::test_indices_interchangeable(&perm.perm, &stab_ind_3);
        };
        let valid_indices_1 = vec![0, 1].into_iter().collect();
        let valid_indices_2 = vec![2, 3].into_iter().collect();
        let valid_indices_3 = vec![4, 5].into_iter().collect();
        let valid_indices = vec![valid_indices_1, valid_indices_2, valid_indices_3];

        let subgroup_gens = SchreierSims::relaxed_schreier_sims(group_gens, valid_indices);
        for subgroup_gen in &subgroup_gens {
            println!("subgroup gen: {:?}", subgroup_gen);
            assert!(membership_test_1(subgroup_gen));
            assert!(membership_test_2(subgroup_gen));
            assert!(membership_test_3(subgroup_gen));
        }

        assert_eq!(subgroup_gens.len(), 4);
    }

    #[test]
    fn test_get_stabilizing_color_gens() {
        let solution_string = "a;b;a;b".to_string();
        let stabilizing_indices = SchreierSims::get_stabilizing_color_gens(&solution_string);
        println!("stabilizing indices: {:?}", stabilizing_indices);
        assert_eq!(stabilizing_indices.len(), 2);
        assert!(stabilizing_indices.contains(&vec![1, 3].into_iter().collect()));
        assert!(stabilizing_indices.contains(&vec![0, 2].into_iter().collect()));
    }
}
