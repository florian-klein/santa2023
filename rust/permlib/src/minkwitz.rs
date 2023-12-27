use crate::factorization::Factorizer;
use crate::permgroups;
use crate::permutation_utils;
use crate::word_length_iter::WordIterator;
use crate::Permutation;
use std::collections::HashMap;

pub struct MinkWitz {}

impl MinkWitz {
    pub fn minkwitz_table(
        genset: &permgroups::GeneratingSet,
        label_to_gen: HashMap<&str, Permutation>,
        max_word_size: usize,
        permutation_size: usize,
    ) -> HashMap<(usize, usize), String> {
        let (_, base) = Factorizer::find_generators_and_base(genset, permutation_size);
        let mut b_i_x: HashMap<(usize, usize), String> = HashMap::new(); // Change the type to String
        let generator_words: Vec<&str> = label_to_gen.keys().map(|&x| x).collect();

        for word_len in 1..=max_word_size {
            let mut word_iterator = WordIterator::new(&generator_words, word_len);
            let _ = word_iterator.next().unwrap();
            while let Some(mut word) = word_iterator.next() {
                let mut i = 0;
                let g = permutation_utils::word_to_perm(&word, label_to_gen.clone());
                while i < base.len() {
                    let x = g.apply_to_single_element(i);
                    if b_i_x.contains_key(&(i, x)) {
                        let w_ = b_i_x.get(&(i, x)).unwrap();
                        if w_.len() <= word_len {
                            b_i_x.insert((i, x), word); // Insert owned String
                            break;
                        } else {
                            let reversed_w_ = w_.chars().rev().collect::<String>();
                            word = reversed_w_ + "." + &word;
                            i += 1;
                        }
                    } else {
                        b_i_x.insert((i, x), word); // Insert owned String
                        break;
                    }
                }
            }
        }
        b_i_x
    }

    pub fn search_factorization(
        goal_perm: &Permutation,
        genset: &permgroups::GeneratingSet,
        label_to_gen: HashMap<&str, Permutation>,
        max_word_size: usize,
        permutation_size: usize,
    ) -> Option<String> {
        let b_i_x = MinkWitz::minkwitz_table(
            genset,
            label_to_gen.clone(),
            max_word_size,
            permutation_size,
        );
        println!("b_i_x: {:?}", b_i_x);
        let labels: Vec<&str> = label_to_gen.keys().map(|&x| x).collect();
        let mut word_iterator = WordIterator::new(&labels, max_word_size);

        let mut shortest_representation: Option<String> = None;

        for _ in 0..1000 {
            let word = word_iterator.next();
            if word.is_none() {
                break;
            }
            let word = word.unwrap();
            let h = permutation_utils::word_to_perm(&word, label_to_gen.clone());
            let h_inv_goal = h.inverse() * goal_perm.clone();

            if let Some(w_prime) = b_i_x.get(&(0, h_inv_goal.apply_to_single_element(0))) {
                let representation = format!("{}.{}", word, w_prime);

                match shortest_representation {
                    Some(ref current_shortest) if representation.len() < current_shortest.len() => {
                        shortest_representation = Some(representation);
                    }
                    None => {
                        shortest_representation = Some(representation);
                    }
                    _ => {}
                }
            }
        }

        shortest_representation
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_minkwitz_small() {
        let mut label_to_gen: HashMap<&str, Permutation> = HashMap::new();
        let perm1 = permutation_utils::parse_permutation_from_cycle("(0,1)", 3);
        let perm2 = permutation_utils::parse_permutation_from_cycle("(1,2)", 3);
        let perm3 = perm1.inverse();
        let perm4 = perm2.inverse();
        label_to_gen.insert("a", perm1.clone());
        label_to_gen.insert("b", perm2.clone());
        label_to_gen.insert("c", perm3.clone());
        label_to_gen.insert("d", perm4.clone());

        let genset =
            permgroups::GeneratingSet::new(vec![perm1.clone(), perm2.clone(), perm3, perm4]);
        let max_word_size = 3;
        let b_i_x = MinkWitz::minkwitz_table(&genset, label_to_gen.clone(), max_word_size, 3);
        println!("b_i_x: {:?}", b_i_x);
        // let goal_perm = perm1.clone() * perm2.clone() * perm1.clone() * perm2.clone();
        // let factorization = MinkWitz::search_factorization(&goal_perm, &genset, label_to_gen.clone(), max_word_size, 3);
        // let evaluated_factorization = permutation_utils::word_to_perm(&factorization.unwrap(), label_to_gen.clone());
        // println!("------------------");
        // println!("goal_perm: {:?}", goal_perm);
        // println!("evaluated_factorization: {:?}", evaluated_factorization);
        // assert_eq!(goal_perm, evaluated_factorization);
        assert_eq!(0, 1);
    }
}
