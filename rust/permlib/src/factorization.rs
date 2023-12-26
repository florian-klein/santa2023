use crate::permgroups;
use crate::Permutation;
use crate::permutation_utils;
use crate::schreier;
use std::collections::HashMap;

pub struct Factorizer {
}

impl Factorizer {
    
    pub fn find_generators_and_base(genset: &permgroups::GeneratingSet, k: usize) -> (Vec<Permutation>, Vec<Vec<usize>>) {
        let mut cur_gens = genset.generators.clone();
        let mut base = vec![];

        for i in 0..k {
            let mut cur_genset = permgroups::GeneratingSet::new(cur_gens.clone());
            cur_genset = Factorizer::sims_filter(&cur_genset);
            let (gens, orbits) = schreier::SchreierVector::get_stab_gens_and_orbits(&cur_genset, i, k);
            println!("gens: {:?}, orbits: {:?}", gens, orbits);

            base.push(orbits);
            if gens.len() == 0 {
                // return base and cur_gense
                return (cur_gens, base);
            }
            cur_gens = gens;
        }

        let cur_genset = permgroups::GeneratingSet::new(cur_gens.clone());
        cur_gens = Factorizer::sims_filter(&cur_genset).generators;

        (cur_gens, base)
    }
    /** 
     * Given a set A \subseteq S_n, there is an effective algorithm to replace A by some B\subseteq S_n satisfying \left<A\right> = \left<B\right> and
     * B is shorter
    */
    fn j_permutation(perm: &Permutation) -> (usize, usize) {
        let mut i = 0;
        let mut j = 0;
        let mut index = 0;
        while index < perm.elements.len() {
            if perm.elements[index] != index {
                i = index;
                j = perm.elements[index];
                break;
            }
            index += 1;
        }
        (i, j)
    }

    pub fn sims_filter(genset: &permgroups::GeneratingSet) -> permgroups::GeneratingSet {
        let mut cur_gens = genset.generators.clone();
        let mut table: HashMap<(usize, usize), Permutation> = HashMap::new();
        let mut new_gens = vec![];
        let mut num_changes = 1;

        while num_changes > 0 {
            num_changes = 0;

            for i in 0..cur_gens.len() {
                let mut cur_gen = cur_gens[i].clone();

                if cur_gen.is_identity() {
                    continue;
                }

                let j_g_i = Factorizer::j_permutation(&cur_gen);

                if let Some(h) = table.get(&(j_g_i.0, j_g_i.1)) {
                    cur_gen = cur_gen.inverse() * h.clone();
                    cur_gens[i] = cur_gen.clone();
                    num_changes += 1;
                } else {
                    table.insert((j_g_i.0, j_g_i.1), cur_gen.clone());
                    new_gens.push(cur_gen.clone());
                }
            }
        }
    
        permgroups::GeneratingSet::new(new_gens)
    }
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_find_generators_for_fixation_group() {
        let perm1 = permutation_utils::parse_permutation_from_cycle("(1,5,7)(2,6,8)", 10);
        let perm2 = permutation_utils::parse_permutation_from_cycle("(1,5)(3,4,8,2)", 10);

        let generating_set = permgroups::GeneratingSet::new(vec![perm1, perm2]);
        let (gens, _) = Factorizer::find_generators_and_base(&generating_set, 10);
        assert_eq!(gens.len(), 5);
    }

    #[test]
    fn test_j_permutation() {
        let perm1 = permutation_utils::parse_permutation_from_cycle("(1,5,7)(2,6,8)", 10);
        assert_eq!(Factorizer::j_permutation(&perm1), (1, 5));
    }

    #[test]
    fn test_sims_filter() {
        let perm1 = permutation_utils::parse_permutation_from_cycle("(1,5,7)(2,6,8)", 10);
        let perm2 = permutation_utils::parse_permutation_from_cycle("(1,5)(3,4,8,2)", 10);
        let perm3 = perm1.clone() * perm2.clone();

        let generating_set = permgroups::GeneratingSet::new(vec![perm1, perm2, perm3]);
        let gens = Factorizer::sims_filter(&generating_set);
        assert_eq!(gens.generators.len(), 3);
    }

    #[test]
    fn test_sims_filter_small(){
        let perm1 = permutation_utils::parse_permutation_from_cycle("(1,2)", 4);
        let perm2 = permutation_utils::parse_permutation_from_cycle("(2,1)", 4);
        let perm3 = permutation_utils::parse_permutation_from_cycle("(3,1)", 4);
        let perm4 = perm1.clone() * perm2.clone();
        let perm5 = perm1.clone() * perm3.clone();
        let perm6 = perm2.clone() * perm3.clone();
        let perm7 = perm1.clone() * perm2.clone() * perm3.clone();
        let generating_set = permgroups::GeneratingSet::new(vec![perm1, perm2, perm3, perm4, perm5, perm6, perm7]);
        let gens = Factorizer::sims_filter(&generating_set);
        for perm in &gens.generators {
            println!("{}", perm);
        }
        assert_eq!(gens.generators.len(), 3);
    }
}
