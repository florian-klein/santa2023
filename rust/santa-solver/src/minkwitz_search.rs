use crate::minkwitz::{self, PermAndWord, TransTable};
use log::info;
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PermWordAssignedIndices {
    pub perm_and_word: PermAndWord,
    pub assigned_indices: HashSet<usize>,
    pub current_index: usize,
}

impl PartialOrd for PermWordAssignedIndices {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return self.perm_and_word.partial_cmp(&other.perm_and_word);
    }
}

impl Ord for PermWordAssignedIndices {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.perm_and_word.cmp(&other.perm_and_word);
    }
}

pub fn minkwitz_djikstra(
    valid_indices: Vec<HashSet<usize>>,
    target: PermAndWord,
    sgs_table: TransTable,
) -> Option<PermAndWord> {
    let mut pq: BinaryHeap<PermWordAssignedIndices> = BinaryHeap::new();
    let initial_target = PermWordAssignedIndices {
        perm_and_word: target.clone(),
        assigned_indices: HashSet::new(),
        current_index: 0,
    };
    pq.push(initial_target);
    /*
     * Try to stabilize each index 1 by 1
     * Start off with index 0, then see where the index is currently mapped to by the permutation
     * Call this point omega. Then look in table for all indices in the same valid_indices group
     * as the current index (lets call this index j), if (j, omega) is in the table.
     * If it is, add the resulting PermAndWord to the priority queue.
     */
    while !pq.is_empty() {
        // return condition, current string is valid target string
        let current = pq.pop().unwrap();
        if minkwitz::MinkwitzTable::check_perm_is_target(
            &current.perm_and_word.perm,
            &valid_indices,
        ) {
            info!("Djikstra found the shortest path in word length to target.");
            return Some(current.perm_and_word.clone());
        }
        // otherwise, check for the current index
        let current_index = current.current_index;
        let omega = current.perm_and_word.perm.p[current_index] - 1;
        // for all elemeents in the same valid_indices group as the current index that are also
        // larger than the current index (since we want to stabilize the indices in order), check
        let current_index_valid_indices = valid_indices
            .iter()
            .find(|valid_indices_group| valid_indices_group.contains(&current_index))
            .unwrap();
        for index in current_index_valid_indices {
            if *index < current_index || current.assigned_indices.contains(index) {
                continue;
            }
            // if the current index and the index we are checking are in the table, add the
            // resulting PermAndWord to the priority queue
            if let Some(table_entry) = sgs_table.get(&(*index, omega)) {
                let mut new_assigned_indices = current.assigned_indices.clone();
                let new_perm_and_word = table_entry.compose(&current.perm_and_word);
                new_assigned_indices.insert(*index);
                let next_index = current_index + 1;
                // create word that is inserted into the pq next
                let new_perm_and_word = PermWordAssignedIndices {
                    perm_and_word: new_perm_and_word,
                    assigned_indices: new_assigned_indices,
                    current_index: next_index,
                };
                pq.push(new_perm_and_word);
            }
        }
    }

    return None;
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use crate::{
        minkwitz::{self, GroupBase, GroupGen, GroupGens, PermAndWord},
        permutation::Permutation,
        schreier,
        testing_utils::TestingUtils,
    };

    #[test]
    fn test_minkwitz_djikstra() {
        let perm1 = Permutation::parse_permutation_from_cycle("(1,2)", 3);
        let perm2 = Permutation::parse_permutation_from_cycle("(1,2,3)", 3);
        let perm1_inv = perm1.inverse();
        let perm2_inv = perm2.inverse();
        let gen1 = GroupGen::new("a".to_string(), perm1.clone());
        let gen2 = GroupGen::new("b".to_string(), perm2.clone());
        let gen1_inv = GroupGen::new("-a".to_string(), perm1_inv.clone());
        let gen2_inv = GroupGen::new(".b".to_string(), perm2_inv.clone());
        let genset = GroupGens::new(vec![gen1_inv, gen1, gen2_inv, gen2]);
        let base = GroupBase::new(vec![0, 1, 2]);
        // build the table
        let sgs_table = minkwitz::MinkwitzTable::build_short_word_sgs(&genset, &base, 100, 10, 100);
        // valid indices is vector of three sets each containing one index
        let valid_indices = vec![
            vec![0].into_iter().collect::<HashSet<usize>>(),
            vec![1].into_iter().collect::<HashSet<usize>>(),
            vec![2].into_iter().collect::<HashSet<usize>>(),
        ];
        println!("valid indices {:?}", valid_indices);
        let target = PermAndWord {
            perm: Permutation::parse_permutation_from_cycle("(2,1,3)", 3),
            word: vec![],
            inverse: vec![],
            news: true,
        };
        let result = super::minkwitz_djikstra(valid_indices, target, sgs_table);
        assert_eq!(result.is_some(), true);
    }

    #[test]
    fn test_minkwitz_colored() {
        let perm1 = Permutation::parse_permutation_from_cycle("(1,5,7)(2,6,8)", 8);
        let perm2 = Permutation::parse_permutation_from_cycle("(1,5)(3,4,8,2)", 8);
        let perm1_inv = perm1.inverse();
        let perm2_inv = perm2.inverse();
        let gen1 = GroupGen::new("a".to_string(), perm1.clone());
        let gen2 = GroupGen::new("b".to_string(), perm2.clone());
        let gen1_inv = GroupGen::new("a_inv".to_string(), perm1_inv.clone());
        let gen2_inv = GroupGen::new("b_inv".to_string(), perm2_inv.clone());
        let gens = GroupGens::new(vec![gen1_inv, gen1, gen2_inv, gen2]);
        let index_to_perm = vec![perm1_inv, perm1.clone(), perm2_inv, perm2.clone()];
        let base = GroupBase {
            elements: vec![0, 1, 2, 3, 4, 5, 6, 7],
        };
        let tt = minkwitz::MinkwitzTable::build_short_word_sgs(&gens, &base, 1000, 20, 10);
        for elm in &tt.table {
            println!("Table entry {:?} is {:?}", elm.0, elm.1);
        }
        let target = perm1.compose(&perm2);
        let target_perm_word = PermAndWord {
            perm: target.clone(),
            word: vec![],
            inverse: vec![],
            news: true,
        };
        let valid_indices =
            schreier::SchreierSims::get_stabilizing_color_gens(&"a;a;a;a;b;b;b;b".to_string());
        println!("Valid indices: {:?}", valid_indices);
        let word = super::minkwitz_djikstra(valid_indices.clone(), target_perm_word, tt);
        let word_elm = word.clone().unwrap().word;
        let perm = TestingUtils::get_perm_from_index_path(&word_elm, &index_to_perm);
        let res = perm.compose(&target);
        assert_eq!(
            minkwitz::MinkwitzTable::check_perm_is_target(&res, &valid_indices),
            true
        );
    }
}
