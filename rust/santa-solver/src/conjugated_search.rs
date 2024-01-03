use crate::groups::DepthLimitedPermutationGroupIterator;
use crate::groups::PermutationGroupIterator;
use crate::permutation::Permutation;
use crate::permutation::PermutationPath;
use crate::puzzle::Move;
use crate::puzzle::Puzzle;
use log::{debug, warn};
use std::collections::HashMap;
use std::collections::VecDeque;

pub fn find_target_c_cycle_in_conjugated_group(
    c_cycle: Permutation,
    c_cycle_path: PermutationPath,
    target_c_cycle: Permutation,
    index_to_move: &Vec<Move>,
) -> Option<PermutationPath> {
    debug!("Searching target c-cycle of length {:?}", c_cycle.p.len());
    let current_path: VecDeque<usize> = c_cycle_path.arr.iter().cloned().collect();
    let current_perm = c_cycle;
    let mut counter = 0;
    let gens = index_to_move
        .iter()
        .map(|x| x.permutation.clone())
        .collect();
    let generator_iterator = DepthLimitedPermutationGroupIterator::new(&gens, 10000);
    for (_, _gen_path) in generator_iterator {
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
        // let gen_inverse = gen_perm.inverse();
        // let inverse_path = gen_perm_to_index.get(&gen_inverse).unwrap();
        // current_perm = gen_perm.compose(&current_perm.compose(&gen_inverse));
        // current_path.push_front(*inverse_path);
        // current_path.push_back(*gen_path);
        counter += 1;
    }
    // debug!("No target c-cycle found");
    None
}

pub fn find_c_cycles_relaxed_search(
    gen_perm_to_index: &HashMap<Permutation, usize>,
    depth: usize,
    target: Permutation,
    puzzle: Puzzle,
) -> Option<Vec<(Permutation, PermutationPath)>> {
    let generator = PermutationGroupIterator::new(&gen_perm_to_index);
    let mut i = 0;
    let mut cycle_length_to_path: Vec<(Permutation, PermutationPath)> = Vec::new();
    let mut index_to_perm: Vec<Permutation> = Vec::new();
    let target_cycles = target.compute_info().cycles_id;
    for move_elm in &puzzle.moves {
        index_to_perm.push(move_elm.permutation.clone());
    }
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
        if tau_info.cycles_id == target_cycles {
            cycle_length_to_path.push((tau, tau_path));
        }
    }
    Some(cycle_length_to_path)
}
