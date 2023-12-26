use crate::permgroups;
use crate::Permutation;
use crate::permutation_utils;
use crate::schreier;

pub struct Factorizer {
}

impl Factorizer {
    
    pub fn find_generators_for_fixation_group(genset: &permgroups::GeneratingSet, k: usize) -> Vec<Permutation> {
        let mut cur_gens = genset.generators.clone();
        for i in 0..k {
            let cur_genset = permgroups::GeneratingSet::new(cur_gens.clone());
            let gens = schreier::SchreierVector::get_stabilizator_gens(&cur_genset, i, k);
            // println!("gens for {}-th point: {:?}", i, gens);
            if gens.len() == 0 {
                println!("No generators found for fixation group");
                return vec![];
            }
            cur_gens = gens;
        }
        cur_gens
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
        let gens = Factorizer::find_generators_for_fixation_group(&generating_set, 10);
        for perm in &gens {
            println!("{}", perm);
        }
        assert_eq!(gens.len(), 0);
    }
}
