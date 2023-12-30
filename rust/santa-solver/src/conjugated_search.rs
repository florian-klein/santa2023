use crate::groups::PermutationGroupIterator;
use crate::permutation::Permutation;
use crate::permutation::PermutationPath;
use log::{debug, warn};
use std::collections::HashMap;
use std::collections::VecDeque;

pub fn find_target_c_cycle_in_conjugated_group(
    c_cycle: Permutation,
    c_cycle_path: PermutationPath,
    target_c_cycle: Permutation,
    gens: &HashMap<Permutation, usize>,
) -> Option<PermutationPath> {
    debug!("Searching target c-cycle of length {:?}", c_cycle.p.len());
    let mut current_path: VecDeque<usize> = c_cycle_path.arr.iter().cloned().collect();
    let mut current_perm = c_cycle;
    let mut counter = 0;
    for (gen_perm, gen_path) in gens {
        if counter % 10000 == 0 {
            debug!(
                "Searching for target c-cycle. Group elements tried: {:?}",
                counter
            );
        }
        if current_perm == target_c_cycle {
            debug!(
                "Found target c-cycle with length with path size: {:?}",
                current_path.len()
            );
            let result_vec_path = current_path.iter().cloned().collect::<Vec<usize>>();
            let result_path = PermutationPath::new(result_vec_path);
            return Some(result_path);
        }
        let gen_inverse = gen_perm.inverse();
        let inverse_path = gens.get(&gen_inverse).unwrap();
        current_perm = gen_perm.compose(&current_perm.compose(&gen_inverse));
        current_path.push_front(*inverse_path);
        current_path.push_back(*gen_path);
        counter += 1;
    }
    debug!("No target c-cycle found");
    None
}

pub fn find_c_cycles_relaxed_search(
    gen_perm_to_index: &HashMap<Permutation, usize>,
    depth: usize,
    target_cycles: HashMap<usize, Vec<Permutation>>, // contains the cycle lengths required to build target
) -> Option<HashMap<usize, PermutationPath>> {
    let generator = PermutationGroupIterator::new(&gen_perm_to_index);
    let mut i = 0;
    let mut cycle_length_to_path: HashMap<usize, PermutationPath> = HashMap::new();
    for (tau_path, tau) in generator {
        i += 1;
        if i % 20000 == 0 {
            debug!("Searching for cycles. Group elements tried: {:?}", i);
        }
        if i >= depth {
            if cycle_length_to_path.len() > 0 {
                return Some(cycle_length_to_path);
            }
            warn!("Aborting after trying {} generators", i);
            return None;
        }
        let tau_info = tau.compute_info();
        // Worst-case: O(#perm_length)
        for cycle_length in &tau_info.cycles_id {
            // check if this permutation contains a permutation length that we also need in target
            if target_cycles.contains_key(cycle_length) {
                // Calculate the power of tau s.t. we have a c-cycle of the given length
                // Achieve this by finding the smallest common multiple of all cycle_lengths but the current one
                let mut lcm = 1;
                for other_cycle_length in &tau_info.cycles_id {
                    if other_cycle_length != cycle_length {
                        lcm = Permutation::lcm_two_nums(lcm, *other_cycle_length);
                    }
                }
                // if the order of other permutations would remove our target cycle, continue
                if lcm % cycle_length == 0 {
                    continue;
                }
                let factorization_length = tau_path.arr.len() * lcm;
                if cycle_length_to_path.contains_key(cycle_length) {
                    // check if we found a shorter factorizationk
                    let prev_path: &PermutationPath =
                        cycle_length_to_path.get(cycle_length).unwrap();
                    if factorization_length < prev_path.arr.len() {
                        debug!(
                            "For the cycle id {:?} with current path len {:?}, we found a new path of length {:?}",
                            cycle_length, prev_path.arr.len(), factorization_length
                        );
                        let mut new_path_indices = vec![];
                        for _ in 0..lcm {
                            new_path_indices.extend(tau_path.arr.clone());
                        }
                        let new_perm_path = PermutationPath::new(new_path_indices);
                        cycle_length_to_path.insert(*cycle_length, new_perm_path);
                    }
                } else {
                    debug!(
                        "For the cycle of length {:?}, we inserted a new path of length {:?}",
                        cycle_length, factorization_length
                    );
                    let mut new_path_indices = vec![];
                    for _ in 0..lcm {
                        new_path_indices.extend(tau_path.arr.clone());
                    }
                    let new_perm_path = PermutationPath::new(new_path_indices);
                    cycle_length_to_path.insert(*cycle_length, new_perm_path);
                }
            }
        }
    }
    None
}
