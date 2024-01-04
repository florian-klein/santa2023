use crate::permutation::Permutation;
use rand;
use rust_schreier::perm::Perm;
use rust_schreier::schreier;

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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::permutation::Permutation;

    #[test]
    fn test_find_base() {
        // a = (1,5,7) (2,6,8),
        // b = (1,5) (3,4,8,2).
        let perm1 = Permutation::parse_permutation_from_cycle("(1,5,7)(2,6,8)", 8);
        let perm2 = Permutation::parse_permutation_from_cycle("(1,5)(3,4,8,2)", 8);
        let gens = vec![perm1, perm2];
        let base = SchreierSims::find_base(gens);
        assert_eq!(base.len(), 5);
    }
}
