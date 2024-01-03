use crate::{groups::PermutationGroupIterator, testing_utils::TestingUtils};
use log::debug;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::permutation::Permutation;
#[derive(Debug)]

pub struct MinkwitzTable {
    table: HashMap<(usize, usize), String>,
}

#[derive(Clone, Debug)]
pub struct GroupBase {
    elements: Vec<usize>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct PermAndWord {
    perm: Permutation,
    word: Vec<usize>,
    pub news: bool,
}

#[derive(Debug)]
pub struct GroupGen {
    name: String,
    perm: Permutation,
}

#[derive(Debug)]
pub struct GroupGens {
    elements: Vec<GroupGen>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransTable {
    table: HashMap<(usize, usize), PermAndWord>,
}

impl GroupGen {
    pub fn new(name: String, perm: Permutation) -> Self {
        GroupGen { name, perm }
    }
}

impl GroupGens {
    pub fn new(elements: Vec<GroupGen>) -> Self {
        let elements: Vec<GroupGen> = elements;
        GroupGens { elements }
    }
    pub fn add(&mut self, gen: GroupGen) {
        self.elements.push(gen);
    }
}

impl GroupBase {
    pub fn new(elements: Vec<usize>) -> Self {
        GroupBase { elements }
    }
}

impl PermAndWord {
    pub fn new(perm: Permutation, word: Vec<usize>) -> Self {
        PermAndWord {
            perm,
            word,
            news: true,
        }
    }
    pub fn get_inverse(&self, base_length: usize) -> Self {
        let mut inverse_word = self.word.clone();
        inverse_word.reverse();
        for i in 0..inverse_word.len() {
            if inverse_word[i] % 2 == 0 {
                inverse_word[i] += 1;
            } else {
                inverse_word[i] -= 1;
            }
        }
        PermAndWord {
            perm: self.perm.inverse(),
            word: inverse_word,
            news: self.news,
        }
    }
    pub fn identity(n: usize) -> Self {
        PermAndWord {
            perm: Permutation::identity(n),
            word: Vec::new(),
            news: true,
        }
    }

    pub fn compose(&self, other: &PermAndWord) -> Self {
        let mut new_word = other.word.clone();
        new_word.extend(&self.word);
        PermAndWord {
            perm: self.perm.compose(&other.perm),
            word: new_word,
            news: true,
        }
    }

    pub fn set_news(&mut self, news: bool) {
        self.news = news;
    }
}

impl TransTable {
    pub fn new() -> Self {
        let table: HashMap<(usize, usize), PermAndWord> = HashMap::new();
        TransTable { table }
    }
    pub fn insert(&mut self, key: (usize, usize), value: PermAndWord) {
        self.table.insert(key, value);
    }
    pub fn get(&self, key: &(usize, usize)) -> Option<&PermAndWord> {
        self.table.get(key)
    }
    pub fn get_mutable(&mut self, key: &(usize, usize)) -> Option<&mut PermAndWord> {
        self.table.get_mut(key)
    }
}

impl MinkwitzTable {
    pub fn factorize_minkwitz(
        gens: &GroupGens,
        base: &GroupBase,
        nu: &TransTable,
        target: &Permutation,
    ) -> Vec<usize> {
        let mut list: Vec<usize> = Vec::new();
        let mut perm = target.clone().inverse();
        let mut index_to_perm: Vec<Permutation> = Vec::new();
        for i in 0..gens.elements.len() {
            index_to_perm.push(gens.elements[i].perm.clone());
        }
        for i in 0..base.elements.len() {
            let omega = perm.p[base.elements[i]] - 1;
            let table_entry = nu.table.get(&(i, omega)).unwrap();
            perm = table_entry.perm.compose(&perm);
            TestingUtils::assert_index_path_equals_permutation(
                &table_entry.word,
                &table_entry.perm,
                &index_to_perm,
            );
            let new_word = &table_entry.word;
            list.extend(new_word);
        }
        if perm != Permutation::identity(target.len()) {
            debug!("We could not find a factorization!");
            return Vec::new();
        }
        debug!("We found a factorization of length {}", list.len());
        return list;
    }

    /* Options:
    n: max number of rounds
    s: reset each s rounds
    w: limit for word size
     */
    pub fn build_short_word_sgs(
        gens: &GroupGens,
        base: &GroupBase,
        n: usize,
        s: usize,
        w: usize,
    ) -> TransTable {
        let mut mu_table = TransTable::new();
        let permutation_size = gens.elements[0].perm.len();
        for i in 0..base.elements.len() {
            mu_table.insert(
                (i, base.elements[i]),
                PermAndWord::identity(gens.elements[0].perm.len()),
            );
        }
        let max = n;
        let mut limit = w;
        let mut count = 0;
        let mut gen_perm_to_index: HashMap<Permutation, usize> = HashMap::new();
        for i in 0..gens.elements.len() {
            gen_perm_to_index.insert(gens.elements[i].perm.clone(), i);
        }
        let group_iterator = PermutationGroupIterator::new(&gen_perm_to_index);
        for (perm_path, perm) in group_iterator {
            if count >= max || Self::is_table_full(permutation_size, &gens, &mu_table) {
                debug!("SGS Generation: Stopping as table is full or max reached");
                break;
            }
            let pw = PermAndWord {
                perm: perm.clone(),
                word: perm_path.arr,
                news: true,
            };
            Self::one_round(&gens, &base, limit, 0, &mut mu_table, pw);
            if (count + 1) % s == 0 {
                debug!("SGS Generation: Starting Improvement Round");
                Self::one_improve(&gens, &base, limit, &mut mu_table);
                if !Self::is_table_full(permutation_size, &gens, &mu_table) {
                    Self::fill_orbits(&gens, &base, limit, &mut mu_table);
                }
                limit = limit * 5 / 4;
            }
            count += 1;
        }
        return mu_table;
    }

    fn is_table_full(n: usize, _gens: &GroupGens, _mu_table: &TransTable) -> bool {
        return false;
    }

    fn one_step(
        gens: &GroupGens,
        base: &GroupBase,
        i: usize,
        t: &PermAndWord,
        mu_table: &mut TransTable,
    ) -> PermAndWord {
        let j = t.perm.p[base.elements[i]] - 1;
        let t_inv = t.get_inverse(gens.elements.len());
        let mut result = PermAndWord::identity(gens.elements[0].perm.len());
        // Let x = g(k_{i+1}) \in O_i. Do we have an entry for B_i(x)?
        if let Some(table_entry) = mu_table.get(&(i, j)) {
            if t.word.len() < table_entry.word.len() {
                // If yes, and w is shorter than the current word in B_i(x), we replace it with w and quit
                mu_table.insert((i, j), t_inv.clone());
                return result;
            }
            // Otherwise, let w' = B_i(x). Replace w with w'^{-1}w and increment i then repeat step 2.
            result = table_entry.compose(t);
        } else {
            // If not, we let B_i(x) be the word w and quit.
            mu_table.insert((i, j), t_inv.clone());
        }
        return result;
    }

    fn one_round(
        gens: &GroupGens,
        base: &GroupBase,
        limit: usize,
        c: usize,
        mu_table: &mut TransTable,
        t: PermAndWord,
    ) {
        let mut i = c;
        let mut t_new = t.clone();
        while i < base.elements.len() && !t_new.perm.is_identity() && t_new.word.len() < limit {
            t_new = Self::one_step(&gens, &base, i, &t_new, mu_table);
            i += 1;
        }
    }

    fn one_improve(
        gens: &GroupGens,
        base: &GroupBase,
        limit: usize,
        mu_table: &mut TransTable,
    ) -> () {
        let mut t = PermAndWord::identity(gens.elements[0].perm.len());
        for j in 0..base.elements.len() {
            // iterate over jth row
            for x in 0..base.elements.len() {
                for y in 0..base.elements.len() {
                    let x_elm = mu_table.get(&(j, x));
                    let y_elm = mu_table.get(&(j, y));
                    if x_elm.is_some() && y_elm.is_some() {
                        let x_elm = x_elm.unwrap();
                        let y_elm = y_elm.unwrap();
                        if x_elm.news || y_elm.news {
                            t = y_elm.compose(&x_elm);
                            Self::one_round(gens, base, limit, j, mu_table, t);
                        }
                    }
                }
            }
            for x in 0..base.elements.len() {
                if let Some(x_elm) = mu_table.get_mutable(&(j, x)) {
                    x_elm.set_news(false);
                }
            }
        }
    }

    pub fn fill_orbits(
        gens: &GroupGens,
        base: &GroupBase,
        limit: usize,
        mu_table: &mut TransTable,
    ) -> () {
        for i in 0..base.elements.len() {
            let mut orbit: Vec<usize> = Vec::new();
            for y in 0..base.elements.len() {
                if let Some(table_entry) = mu_table.get(&(i, y)) {
                    let j = table_entry.perm.p[base.elements[i]] - 1;
                    if !orbit.contains(&j) {
                        orbit.push(j);
                    }
                }
            }
            for j in i + 1..base.elements.len() {
                for k in 0..base.elements.len() {
                    let x = mu_table.get(&(j, k));
                    if !x.is_some() {
                        continue;
                    }
                    let x1 = x.unwrap().get_inverse(gens.elements.len());
                    let orbit_x: Vec<usize> =
                        orbit.iter().map(|it| x.unwrap().perm.p[*it] - 1).collect();
                    let new_pts: Vec<usize> = orbit_x
                        .iter()
                        .filter(|it| !orbit.contains(it))
                        .map(|it| *it)
                        .collect();
                    for p in new_pts {
                        if let Some(table_entry) = mu_table.get(&(i, x1.perm.p[p] - 1)) {
                            let t1 = table_entry.compose(&x1);
                            if t1.word.len() < limit {
                                mu_table.insert((i, p), t1);
                            }
                        }
                    }
                }
            }
        }
    }
}

mod test {
    use super::MinkwitzTable;
    use crate::{permutation::Permutation, testing_utils::TestingUtils};

    fn is_valid_sgs(
        tt: &super::TransTable,
        base: &super::GroupBase,
        index_to_perm: &Vec<Permutation>,
    ) {
        let mut result = true;
        for i in 0..base.elements.len() {
            let p = tt.get(&(i, i)).unwrap().perm.clone();
            if !p.is_identity() {
                println!("p {:?} is not identity", (i, i));
                result = false;
            }
            for j in 0..base.elements.len() {
                if let Some(table_entry) = tt.get(&(i, j)) {
                    let p = table_entry.perm.clone();
                    for k in 0..i {
                        if p.p[base.elements[k]] - 1 != base.elements[k] {
                            result = false;
                            println!(
                                "Table entry {:?} is not valid due to base elements as p {:?}, baselmk {:?}",
                                (i, j),
                                p,
                                base.elements[k]
                            );
                        }
                    }
                    if p.p[j] - 1 != base.elements[i] {
                        println!("Table entry {:?} is not valid", (i, j));
                        result = false;
                    }
                    TestingUtils::assert_index_path_equals_permutation(
                        &table_entry.word,
                        &table_entry.perm,
                        &index_to_perm,
                    )
                }
            }
        }
        assert!(result);
    }

    #[test]
    fn test_group_gens() {
        let perm1 = Permutation::parse_permutation_from_cycle("(1,2)", 3);
        let perm2 = Permutation::parse_permutation_from_cycle("(2,3)", 3);
        let gen1 = super::GroupGen::new("a".to_string(), perm1.clone());
        assert_eq!(gen1.name, "a".to_string());
        assert_eq!(gen1.perm, perm1);
        let gen2 = super::GroupGen::new("b".to_string(), perm2.clone());
        assert_eq!(gen2.name, "b".to_string());
        assert_eq!(gen2.perm, perm2);
        let gens = super::GroupGens::new(vec![gen1, gen2]);
        assert_eq!(gens.elements.len(), 2);
    }

    #[test]
    fn test_generate_minkwitz_table() {
        // 1) Create Generating Set
        let perm1 = Permutation::parse_permutation_from_cycle("(1,5,7)(2,6,8)", 8);
        let perm2 = Permutation::parse_permutation_from_cycle("(1,5)(3,4,8,2)", 8);
        let perm1_inv = perm1.inverse();
        let perm2_inv = perm2.inverse();

        let index_to_gen = vec![
            perm1_inv.clone(),
            perm1.clone(),
            perm2_inv.clone(),
            perm2.clone(),
        ];

        let gen1 = super::GroupGen::new("a".to_string(), perm1);
        let gen2 = super::GroupGen::new("b".to_string(), perm2);
        let gen1_inv = super::GroupGen::new("a_inv".to_string(), perm1_inv);
        let gen2_inv = super::GroupGen::new("b_inv".to_string(), perm2_inv);
        let gens = super::GroupGens::new(vec![gen1_inv, gen1, gen2_inv, gen2]);
        // 2) Create Base
        let base = super::GroupBase {
            elements: vec![0, 1, 2, 3, 4, 5, 6, 7],
        };
        let tt = MinkwitzTable::build_short_word_sgs(&gens, &base, 100, 10, 1000);
        for i in 0..base.elements.len() {
            for j in 0..base.elements.len() {
                if i == j {
                    continue;
                }
                if let Some(table_entry) = tt.get(&(i, j)) {
                    println!("Table entry {:?} is {:?}", (i, j), table_entry);
                }
            }
        }
        is_valid_sgs(&tt, &base, &index_to_gen);
    }

    #[test]
    fn test_factorize_m() {
        let perm1 = Permutation::parse_permutation_from_cycle("(1,5,7)(2,6,8)", 8);
        let perm2 = Permutation::parse_permutation_from_cycle("(1,5)(3,4,8,2)", 8);
        let perm1_inv = perm1.inverse();
        let perm2_inv = perm2.inverse();

        let index_to_gen = vec![perm1_inv.clone(), perm1.clone(), perm2_inv.clone()];

        let gen1 = super::GroupGen::new("a".to_string(), perm1.clone());
        let gen2 = super::GroupGen::new("b".to_string(), perm2.clone());
        let gen1_inv = super::GroupGen::new("-a".to_string(), perm1_inv);
        let gen2_inv = super::GroupGen::new("-b".to_string(), perm2_inv);

        let gens = super::GroupGens::new(vec![gen1_inv, gen1, gen2_inv, gen2]);
        let base = super::GroupBase {
            elements: vec![0, 1, 2, 3, 4, 5, 6, 7],
        };
        let tt = MinkwitzTable::build_short_word_sgs(&gens, &base, 100, 10, 1000);
        is_valid_sgs(&tt, &base, &index_to_gen);
        for elm in &tt.table {
            println!("Table entry {:?} is {:?}", elm.0, elm.1);
        }
        let target = perm1.compose(&perm2);
        let fact = MinkwitzTable::factorize_minkwitz(&gens, &base, &tt, &target);
        TestingUtils::assert_index_path_equals_permutation(&fact, &target, &index_to_gen);
        println!("Factorization: {:?}", fact);
    }
}
