use crate::permgroups;
use crate::permutation_utils;
use crate::Permutation;

#[derive(Debug, PartialEq)]
pub struct SchreierVector {
    pub vector: Vec<Option<(usize, usize)>>,
    k: usize,
}

impl SchreierVector {
    pub fn new(vector: Vec<Option<(usize, usize)>>, k: usize) -> SchreierVector {
        SchreierVector { vector, k }
    }

    /**
     * For each point it gives: optional coset representative (perm moving alpha) to that point
     */
    pub fn orbit_transversal(
        gens: &permgroups::GeneratingSet,
        alpha: usize,
        k: usize,
    ) -> Vec<Option<Permutation>> {
        let mut result = vec![None; k];
        let mut old_level: Vec<usize> = vec![];
        let mut new_level: Vec<usize> = vec![alpha];

        result[alpha] = Some(Permutation::identity(k));

        while new_level.len() > 0 {
            std::mem::swap(&mut new_level, &mut old_level);
            new_level = vec![];
            for x in &old_level {
                for item in &gens.generators {
                    let y = item.apply_to_single_element(*x);
                    if result[y].is_none() {
                        let permutation_x = result[*x].as_ref().unwrap().clone();
                        result[y] = Some(item.clone() * permutation_x);
                        new_level.push(y);
                    }
                }
            }
        }
        result
    }

    /**
     * Returns generating set for stabilizator of alpha (mapping alpha to its correct index)
     */
    pub fn get_stab_gens_and_orbits(
        gens: &permgroups::GeneratingSet,
        alpha: usize,
        k: usize,
    ) -> (Vec<Permutation>, Vec<usize>) {
        let mut result = vec![];
        let reps = SchreierVector::orbit_transversal(gens, alpha, k);
        let mut orbits = vec![];
        for (i, rep_i) in reps.iter().enumerate() {
            if let Some(rep_i) = rep_i {
                orbits.push(i);
                for item in &gens.generators {
                    let left = item.apply_to_single_element(i);
                    let left_perm = reps[left].as_ref().unwrap().inverse();
                    let perm = left_perm * item.clone() * rep_i.clone();
                    if !perm.is_identity() {
                        result.push(perm);
                    }
                }
            }
        }

        (result, orbits)
    }
}

/* Breadth-first search to determine the orbit of alpha point and transversal.
 * For each point it gives: an optional tuple (index of generator, preimage under that perm).
*/
pub fn get_schreier_vector(
    gens: &permgroups::GeneratingSet,
    perm_length: usize,
    alpha: usize,
) -> SchreierVector {
    let mut result = vec![None; perm_length];
    let mut old_level: Vec<usize> = vec![];
    let mut new_level: Vec<usize> = vec![alpha];

    result[alpha] = Some((0, 0 as usize));

    while new_level.len() > 0 {
        std::mem::swap(&mut new_level, &mut old_level);
        new_level = vec![];
        for x in &old_level {
            for (i, item) in gens.generators.iter().enumerate() {
                let y = item.apply_to_single_element(*x);
                if result[y].is_none() {
                    result[y] = Some((i, *x));
                    new_level.push(y);
                }
            }
        }
    }

    SchreierVector {
        vector: result,
        k: perm_length,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_schreier_vector() {
        let perm1 = Permutation::new(vec![0, 5, 1, 2, 7, 3, 6, 9, 8, 4]);
        let perm2 = Permutation::new(vec![0, 6, 2, 3, 4, 5, 7, 1, 8, 9]);
        let generating_set = permgroups::GeneratingSet::new(vec![perm1, perm2]);
        let schreier_vector_1 = get_schreier_vector(&generating_set, 10, 1);
        let expected_schreier_vector = SchreierVector::new(
            vec![
                None,
                Some((0, 0)),
                Some((0, 3)),
                Some((0, 5)),
                Some((0, 9)),
                Some((0, 1)),
                Some((1, 1)),
                Some((1, 6)),
                None,
                Some((0, 7)),
            ],
            10,
        );
        assert_eq!(schreier_vector_1, expected_schreier_vector);
    }

    #[test]
    fn test_get_stab_gens_and_orbits() {
        let perm1 = permutation_utils::parse_permutation_from_cycle("(1,5,7)(2,6,8)", 10);
        let perm2 = permutation_utils::parse_permutation_from_cycle("(1,5)(3,4,8,2)", 10);

        let generating_set = permgroups::GeneratingSet::new(vec![perm1, perm2]);
        let (stabilizator_gens, orbits) =
            SchreierVector::get_stab_gens_and_orbits(&generating_set, 1, 10);
        assert_eq!(stabilizator_gens.len(), 3);
        assert_eq!(orbits, vec![1, 5, 7])
    }

    #[test]
    fn test_get_stab_gens_and_orbits_long() {
        let perm1 = permutation_utils::parse_permutation_from_cycle(
            "(0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19)",
            20,
        );
        let perm2 = permutation_utils::parse_permutation_from_cycle("(0,3)(1,2)", 20);

        let generating_set = permgroups::GeneratingSet::new(vec![perm1, perm2]);
        let (stabilizator_gens, _) =
            SchreierVector::get_stab_gens_and_orbits(&generating_set, 1, 20);
        assert_eq!(stabilizator_gens.len(), 20);
    }
}
