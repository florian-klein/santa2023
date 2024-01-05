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

    #[test]
    fn test_find_base_rubik_small() {
        // const MiniRubik = """
        // F: (9 10 12 11)(3 13 22 8)(4 15 21 6)
        // B: (17 18 20 19)(1 7 24 14)(2 5 23 16)
        // U: (1 2 4 3)(9 5 17 13)(10 6 18 14)
        // D: (21 22 24 23)(11 15 19 7)(12 16 20 8)
        // L: (5 6 8 7)(9 21 20 1)(11 23 18 3)
        // R: (13 14 16 15)(10 2 19 22)(12 4 17 24)
        // """
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
}
