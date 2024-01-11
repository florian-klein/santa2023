use crate::minkwitz::{self, PermAndWord, TransTable};
use log::error;
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
        let self_word_len = self.perm_and_word.word.len();
        let other_word_len = other.perm_and_word.word.len();

        Some(self_word_len.cmp(&other_word_len))
    }
}

impl Ord for PermWordAssignedIndices {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_word_len = self.perm_and_word.word.len();
        let other_word_len = other.perm_and_word.word.len();

        self_word_len.cmp(&other_word_len)
    }
}

pub fn get_stabilized_up_to_index(
    perm_word_assigned_indices: &PermAndWord,
    valid_indices: &Vec<HashSet<usize>>,
) -> usize {
    // returns the first index that is not stabilized setwise
    let mut stabilized_up_to_index = perm_word_assigned_indices.perm.len();
    // find the smallest index not stabilized
    for valid_indices_group in valid_indices {
        for index in valid_indices_group {
            let omega = perm_word_assigned_indices.perm.p[*index] - 1;
            if !valid_indices_group.contains(&omega) {
                stabilized_up_to_index = std::cmp::min(stabilized_up_to_index, *index);
            }
        }
    }
    return stabilized_up_to_index;
}

pub fn minkwitz_djikstra(
    valid_indices: Vec<HashSet<usize>>,
    target: PermAndWord,
    sgs_table: TransTable,
    limit: usize,
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
    let mut elements_looked_at = 0;
    let mut max_index_seen = 0;
    while !pq.is_empty() {
        elements_looked_at += 1;
        if elements_looked_at % 100 == 0 {
            info!("Djikstra has looked at {:?} elements", elements_looked_at);
            info!(
                "Current max index up to which perm is stabilized: {:?}",
                max_index_seen
            );
            info!("Current queue length: {:?}", pq.len());
            break;
        }
        // return condition, current string is valid target string
        let current = pq.pop().unwrap();
        max_index_seen = std::cmp::max(max_index_seen, current.current_index);
        if minkwitz::MinkwitzTable::check_perm_is_target(
            &current.perm_and_word.perm,
            &valid_indices,
        ) {
            info!("Djikstra found the shortest path in word length to target.");
            info!("In the end, the queue had a length of {:?}", pq.len());
            info!("Djikstra looked at {:?} elements", elements_looked_at);
            let mut count_of_elements_not_at_own_place = 0;
            for (index, elm) in current.perm_and_word.perm.p.iter().enumerate() {
                if index != *elm - 1 {
                    count_of_elements_not_at_own_place += 1;
                }
            }
            info!(
                "Djikstra found perm with {:?} elements not at their own place",
                count_of_elements_not_at_own_place
            );
            return Some(current.perm_and_word.clone());
        }
        // otherwise, check for the current index
        let current_index = current.current_index;
        // for all elemeents in the same valid_indices group as the current index that are also
        // larger than the current index (since we want to stabilize the indices in order), check
        let current_index_valid_indices = valid_indices
            .iter()
            .find(|valid_indices_group| valid_indices_group.contains(&current_index))
            .unwrap();
        let find_index_location = current.perm_and_word.perm.inverse();
        for index in current_index_valid_indices {
            let index_omega = find_index_location.p[*index] - 1;
            /*
             * Note: It could happen that for all elements that have not yet been assigned, that
             * there is no permutation that directly maps the required element to the correct
             * position. Therefore, we need to find a mapping that maps the index we require to an
             * index that we have a mapping from to the goal index
             */
            if current.assigned_indices.contains(index) {
                continue;
            }
            // if we reach this position here, we know that the index has not yet been assigned
            // therefore, it is at a position from which we need a mapping to our goal index
            // if there is such a mapping (see if condition, everything is fine )
            // if we do not have such a mapping, we need to take a look at all other indices in the
            // table that map to our current_index and for each element that is still not assigned
            // in the set, check if we can map the element from its current index to an index
            // reachable
            // if the current index and the index we are checking are in the table, add the
            // resulting PermAndWord to the priority queue
            if let Some(table_entry) = sgs_table.get(&(current_index, index_omega)) {
                // if cur_assigned_count > limit {
                //     continue;
                // }
                let mut new_assigned_indices = current.assigned_indices.clone();
                let new_perm_and_word = current.perm_and_word.compose(&table_entry.get_inverse());
                let next_index =
                    self::get_stabilized_up_to_index(&new_perm_and_word, &valid_indices);
                for i in 0..next_index {
                    new_assigned_indices.insert(new_perm_and_word.perm.p[i] - 1);
                }
                // create word that is inserted into the pq next
                let new_perm_and_word = PermWordAssignedIndices {
                    perm_and_word: new_perm_and_word,
                    assigned_indices: new_assigned_indices,
                    current_index: next_index,
                };
                pq.push(new_perm_and_word);
            } else {
                let mut reachable_table_entries: Vec<usize> = Vec::new();
                // find all indics that can be mapped to current_index
                for i in current_index..current.perm_and_word.perm.len() {
                    if sgs_table.table.contains_key(&(current_index, i)) {
                        reachable_table_entries.push(i);
                    }
                }

                // for each index that can be mapped to our current_index, check if can map the
                // element at index index_omega (this element is index) to any of those indices
                // todo: this is recursive, solve this somehow :eyes:
                for reaches_cur_index in &reachable_table_entries {
                    if let Some(table_entry) = sgs_table.get(&(*reaches_cur_index, index_omega)) {
                        error!("table has {:?}", (reaches_cur_index, index_omega));
                        let new_perm_and_word =
                            current.perm_and_word.compose(&table_entry.get_inverse());
                        let next_index =
                            self::get_stabilized_up_to_index(&new_perm_and_word, &valid_indices);
                        let mut new_assigned_indices = current.assigned_indices.clone();
                        for i in 0..next_index {
                            new_assigned_indices.insert(new_perm_and_word.perm.p[i] - 1);
                        }
                        // create word that is inserted into the pq next
                        let new_perm_and_word = PermWordAssignedIndices {
                            perm_and_word: new_perm_and_word,
                            assigned_indices: new_assigned_indices,
                            current_index: next_index,
                        };
                        pq.push(new_perm_and_word);
                        return None;
                    }
                }
            }
        }
    }
    info!(
        "Djikstra did not find a path to the target permutation. Failed at index {:?} with keys: \n {:?}",
        max_index_seen, sgs_table.table.keys()
    );
    let mut keys_seen: HashSet<usize> = HashSet::new();
    for key in sgs_table.table.keys() {
        keys_seen.insert(key.0);
        keys_seen.insert(key.1);
    }
    for i in 0..target.perm.len() {
        if !keys_seen.contains(&i) {
            error!("Missing key {:?}", i);
        }
    }
    // assert that each element is some
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
        let sgs_table =
            minkwitz::MinkwitzTable::build_short_word_sgs(&genset, &base, 100, 10, 100, None);
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
        let result = super::minkwitz_djikstra(valid_indices, target, sgs_table, 1000);
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
        let tt = minkwitz::MinkwitzTable::build_short_word_sgs(&gens, &base, 1000, 20, 10, None);
        for elm in &tt.table {
            println!("Table entry {:?} is {:?}", elm.0, elm.1);
        }
        let target = perm1.compose(&perm2);
        let target_perm_word = PermAndWord {
            perm: target.inverse().clone(),
            word: vec![],
            inverse: vec![],
            news: true,
        };
        let valid_indices =
            schreier::SchreierSims::get_stabilizing_color_gens(&"a;a;a;a;b;b;b;b".to_string());
        println!("Valid indices: {:?}", valid_indices);
        let word = super::minkwitz_djikstra(valid_indices.clone(), target_perm_word, tt, 1000);
        let word_elm = word.clone().unwrap().word;
        let perm = TestingUtils::get_perm_from_index_path(&word_elm, &index_to_perm);
        let res = target.compose(&perm);
        assert_eq!(
            minkwitz::MinkwitzTable::check_perm_is_target(&res, &valid_indices),
            true
        );
    }

    #[test]
    fn test_stabilized_up_to_index() {
        let target_perm_word = PermAndWord {
            perm: Permutation::parse_permutation_from_str_arr("[4,3,2,6,1,8,7,5]".to_string()),
            word: vec![],
            inverse: vec![],
            news: true,
        };
        let valid_indices =
            schreier::SchreierSims::get_stabilizing_color_gens(&"a;a;a;a;b;b;b;b".to_string());
        let stabilized_up_to_index =
            super::get_stabilized_up_to_index(&target_perm_word, &valid_indices);
        assert_eq!(stabilized_up_to_index, 3);
    }
}
