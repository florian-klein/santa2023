use crate::groups::PermutationGroupIterator;
use log::debug;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::hash::{Hash, Hasher};
use std::io::{self, BufRead, Write};

use crate::permutation::Permutation;
#[derive(Debug)]

pub struct MinkwitzTable {
    pub table: HashMap<(usize, usize), String>,
}

#[derive(Clone, Debug)]
pub struct GroupBase {
    pub elements: Vec<usize>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PermAndWord {
    pub perm: Permutation,
    pub word: Vec<usize>,
    pub news: bool,
    pub inverse: Vec<usize>,
}

#[derive(Debug)]
pub struct GroupGen {
    pub name: String,
    pub perm: Permutation,
}

#[derive(Debug)]
pub struct GroupGens {
    elements: Vec<GroupGen>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransTable {
    pub table: HashMap<(usize, usize), PermAndWord>,
    pub group_elements_processed: usize,
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
    pub fn load_from_file(path: &str) -> Self {
        let file = fs::File::open(path).unwrap();
        // Create a buffered reader to read lines
        let reader = io::BufReader::new(file);
        let Some(Ok(first_line)) = reader.lines().next() else {
            panic!("Could not read base string of file");
        };
        let base: Vec<usize> = first_line
            .split(".")
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        GroupBase { elements: base }
    }

    pub fn write_to_file(&self, path: &str) {
        let mut wtr = csv::Writer::from_path(path).unwrap();
        let mut base_str = String::new();
        for i in 0..self.elements.len() {
            base_str.push_str(&self.elements[i].to_string());
            if i < self.elements.len() - 1 {
                base_str.push_str(".");
            }
        }
        wtr.write_record(&[base_str]).unwrap();
        wtr.flush().unwrap();
    }
}

impl PermAndWord {
    pub fn new(perm: Permutation, word: Vec<usize>) -> Self {
        PermAndWord {
            perm,
            word,
            news: true,
            inverse: Vec::new(),
        }
    }

    pub fn new_with_inverse(perm: Permutation, word: Vec<usize>, inverse: Vec<usize>) -> Self {
        PermAndWord {
            perm,
            word,
            news: true,
            inverse,
        }
    }
    pub fn get_inverse(&self) -> Self {
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
            inverse: self.word.clone(),
        }
    }
    pub fn identity(n: usize) -> Self {
        PermAndWord {
            perm: Permutation::identity(n),
            word: Vec::new(),
            news: true,
            inverse: Vec::new(),
        }
    }

    pub fn compose(&self, other: &PermAndWord) -> Self {
        let mut new_word = other.word.clone();
        new_word.extend(&self.word);
        PermAndWord {
            perm: self.perm.compose(&other.perm),
            word: new_word,
            news: true,
            inverse: Vec::new(),
        }
    }

    pub fn set_news(&mut self, news: bool) {
        self.news = news;
    }
}

impl Hash for PermAndWord {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.perm.hash(state);
    }
}

impl PartialEq for PermAndWord {
    fn eq(&self, other: &Self) -> bool {
        self.perm == other.perm
    }
}

impl Eq for PermAndWord {}

impl Ord for PermAndWord {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.word.len().cmp(&other.word.len())
    }
}

impl PartialOrd for PermAndWord {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl TransTable {
    pub fn new() -> Self {
        let table: HashMap<(usize, usize), PermAndWord> = HashMap::new();
        TransTable {
            table,
            group_elements_processed: 0,
        }
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
    pub fn read_from_file(path: &str) -> Self {
        let file = fs::File::open(path).unwrap();
        let reader = io::BufReader::new(file);
        let table: HashMap<(usize, usize), PermAndWord> =
            bincode::deserialize_from(reader).unwrap();
        TransTable {
            table,
            // TODO: also read this from file
            group_elements_processed: 0,
        }
    }
    pub fn write_to_file(&self, path: &str) -> () {
        let file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(path)
            .unwrap();
        let mut file_writer = io::BufWriter::new(file);
        let _ = bincode::serialize_into(&mut file_writer, &self);
    }
}

impl MinkwitzTable {
    pub fn check_perm_is_target(perm: &Permutation, valid_indices: &Vec<HashSet<usize>>) -> bool {
        // check that for each set in valid_indices, the entry at index is in the set
        for index_set in valid_indices {
            for index in index_set {
                let omega = perm.p[*index] - 1;
                if !index_set.contains(&omega) {
                    return false;
                }
            }
        }
        return true;
    }
    pub fn factorize_minkwitz(
        _gens: &GroupGens,
        base: &GroupBase,
        nu: &TransTable,
        target: &Permutation,
        valid_indices: &Vec<HashSet<usize>>,
    ) -> Vec<usize> {
        // sort valid indices based on the first element in the set
        let mut sorted_valid_indices: Vec<Vec<usize>> = valid_indices
            .iter()
            .map(|it| it.iter().map(|it| *it).collect())
            .collect();
        // sort each vector in sorted_valid_indices, then sort the vectors based on the first element
        for i in 0..sorted_valid_indices.len() {
            sorted_valid_indices[i].sort();
        }
        sorted_valid_indices.sort_by(|a, b| {
            let a_first = a.iter().next().unwrap();
            let b_first = b.iter().next().unwrap();
            a_first.cmp(b_first)
        });
        let mut list: Vec<usize> = Vec::new();
        let mut perm = target.inverse().clone();
        for i in 0..base.elements.len() {
            let omega = perm.p[base.elements[i]] - 1;
            // testing
            let mut set_where_i_is_in = None;
            for set in sorted_valid_indices.iter() {
                if set.contains(&i) {
                    set_where_i_is_in = Some(set);
                    println!("set: {:?}", set);
                    break;
                }
            }
            for elm in set_where_i_is_in.unwrap() {
                let _subelm = nu.table.get(&(*elm, omega));
            }
            // testing
            let table_entry = nu.table.get(&(i, omega));
            if !table_entry.is_some() {
                debug!("We could not find a factorization!");
                return Vec::new();
            }
            let table_entry = table_entry.unwrap();
            perm = table_entry.perm.compose(&perm);
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
                mu_table.group_elements_processed = count;
                break;
            }
            let pw = PermAndWord {
                perm: perm.clone(),
                word: perm_path.arr,
                news: true,
                inverse: Vec::new(),
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

    fn is_table_full(_n: usize, _gens: &GroupGens, _mu_table: &TransTable) -> bool {
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
        let t_inv = t.get_inverse();
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
                            let t = y_elm.compose(&x_elm);
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
        _gens: &GroupGens,
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
                    let x1 = x.unwrap().get_inverse();
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

pub fn is_valid_sgs(tt: &TransTable, base: &GroupBase) {
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
            }
        }
    }
    assert!(result);
}
mod test {

    #[test]
    fn test_perm_and_word_compose() {
        let gen1 = super::GroupGen::new(
            "a".to_string(),
            super::Permutation::parse_permutation_from_cycle("(1,2)", 3),
        );
        let gen2 = super::GroupGen::new(
            "b".to_string(),
            super::Permutation::parse_permutation_from_cycle("(2,3)", 3),
        );
        let gen1_inv = super::GroupGen::new("a_inv".to_string(), gen1.perm.inverse());
        let gen2_inv = super::GroupGen::new("b_inv".to_string(), gen2.perm.inverse());
        let index_to_gen = vec![
            gen1_inv.perm.clone(),
            gen1.perm.clone(),
            gen2_inv.perm.clone(),
            gen2.perm.clone(),
        ];
        let pw = super::PermAndWord::new(gen1.perm.clone(), vec![1]);
        let pw2 = super::PermAndWord::new(gen2.perm.clone(), vec![3]);
        let pw3 = pw.compose(&pw2);
        let expected_perm = gen1.perm.compose(&gen2.perm);
        assert_eq!(pw3.perm, expected_perm);
        let other_word = pw3.word.clone();
        crate::testing_utils::TestingUtils::assert_index_path_equals_permutation(
            &other_word,
            &pw3.perm,
            &index_to_gen,
        );
    }

    #[allow(dead_code)]
    #[test]
    fn test_group_gens() {
        let perm1 = super::Permutation::parse_permutation_from_cycle("(1,2)", 3);
        let perm2 = super::Permutation::parse_permutation_from_cycle("(2,3)", 3);
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
        let perm1 = super::Permutation::parse_permutation_from_cycle("(1,5,7)(2,6,8)", 8);
        let perm2 = super::Permutation::parse_permutation_from_cycle("(1,5)(3,4,8,2)", 8);
        let perm1_inv = perm1.inverse();
        let perm2_inv = perm2.inverse();

        let _index_to_gen = vec![
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
        let tt = super::MinkwitzTable::build_short_word_sgs(&gens, &base, 100, 10, 1000);
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
        super::is_valid_sgs(&tt, &base);
    }

    #[test]
    fn test_factorize_m() {
        let perm1 = super::Permutation::parse_permutation_from_cycle("(1,5,7)(2,6,8)", 8);
        let perm2 = super::Permutation::parse_permutation_from_cycle("(1,5)(3,4,8,2)", 8);
        let perm1_inv = perm1.inverse();
        let perm2_inv = perm2.inverse();

        let index_to_gen = vec![
            perm1_inv.clone(),
            perm1.clone(),
            perm2_inv.clone(),
            perm2.clone(),
        ];

        let gen1 = super::GroupGen::new("a".to_string(), perm1.clone());
        let gen2 = super::GroupGen::new("b".to_string(), perm2.clone());
        let gen1_inv = super::GroupGen::new("-a".to_string(), perm1_inv);
        let gen2_inv = super::GroupGen::new("-b".to_string(), perm2_inv);

        let gens = super::GroupGens::new(vec![gen1_inv, gen1, gen2_inv, gen2]);
        let base = super::GroupBase {
            elements: vec![0, 1, 2, 3, 4, 5, 6, 7],
        };
        let tt = super::MinkwitzTable::build_short_word_sgs(&gens, &base, 100, 10, 1000);
        super::is_valid_sgs(&tt, &base);
        for elm in &tt.table {
            println!("Table entry {:?} is {:?}", elm.0, elm.1);
        }
        let target = perm1.compose(&perm2);
        // 7, 3, 4, 2, 5, 8, 1, 6
        let valid_indices = crate::schreier::SchreierSims::get_stabilizing_color_gens(
            &"7;3;4;2;5;8;1;6".to_string(),
        );
        let fact =
            super::MinkwitzTable::factorize_minkwitz(&gens, &base, &tt, &target, &valid_indices);
        crate::testing_utils::TestingUtils::assert_index_path_equals_permutation(
            &fact,
            &target,
            &index_to_gen,
        );
        println!("Factorization: {:?}", fact);
    }

    #[test]
    fn test_rubik_small_and_base_not_full() {
        let perm_f = super::Permutation::parse_permutation_from_cycle(
            "(9,10,12,11)(3,13,22,8)(4,15,21,6)",
            24,
        );
        let perm_b = super::Permutation::parse_permutation_from_cycle(
            "(17,18,20,19)(1,7,24,14)(2,5,23,16)",
            24,
        );
        let perm_u = super::Permutation::parse_permutation_from_cycle(
            "(1,2,4,3)(9,5,17,13)(10,6,18,14)",
            24,
        );
        let perm_d = super::Permutation::parse_permutation_from_cycle(
            "(21,22,24,23)(11,15,19,7)(12,16,20,8)",
            24,
        );
        let perm_l = super::Permutation::parse_permutation_from_cycle(
            "(5,6,8,7)(9,21,20,1)(11,23,18,3)",
            24,
        );
        let perm_r = super::Permutation::parse_permutation_from_cycle(
            "(13,14,16,15)(10,2,19,22)(12,4,17,24)",
            24,
        );
        let perm_f_inv = perm_f.inverse();
        let perm_b_inv = perm_b.inverse();
        let perm_u_inv = perm_u.inverse();
        let perm_d_inv = perm_d.inverse();
        let perm_l_inv = perm_l.inverse();
        let perm_r_inv = perm_r.inverse();

        let index_to_gen = vec![
            perm_f_inv.clone(),
            perm_f.clone(),
            perm_b_inv.clone(),
            perm_b.clone(),
            perm_u_inv.clone(),
            perm_u.clone(),
            perm_d_inv.clone(),
            perm_d.clone(),
            perm_l_inv.clone(),
            perm_l.clone(),
            perm_r_inv.clone(),
            perm_r.clone(),
        ];

        let gen_f = super::GroupGen::new("F".to_string(), perm_f.clone());
        let gen_b = super::GroupGen::new("B".to_string(), perm_b.clone());
        let gen_u = super::GroupGen::new("U".to_string(), perm_u.clone());
        let gen_d = super::GroupGen::new("D".to_string(), perm_d.clone());
        let gen_l = super::GroupGen::new("L".to_string(), perm_l.clone());
        let gen_r = super::GroupGen::new("R".to_string(), perm_r.clone());
        let gen_f_inv = super::GroupGen::new("F_inv".to_string(), perm_f_inv);
        let gen_b_inv = super::GroupGen::new("B_inv".to_string(), perm_b_inv);
        let gen_u_inv = super::GroupGen::new("U_inv".to_string(), perm_u_inv);
        let gen_d_inv = super::GroupGen::new("D_inv".to_string(), perm_d_inv);
        let gen_l_inv = super::GroupGen::new("L_inv".to_string(), perm_l_inv);
        let gen_r_inv = super::GroupGen::new("R_inv".to_string(), perm_r_inv);

        let gens = super::GroupGens::new(vec![
            gen_f_inv, gen_f, gen_b_inv, gen_b, gen_u_inv, gen_u, gen_d_inv, gen_d, gen_l_inv,
            gen_l, gen_r_inv, gen_r,
        ]);

        let base = super::GroupBase {
            elements: vec![0, 1, 2, 3, 20, 21, 22, 23],
        };
        let tt = super::MinkwitzTable::build_short_word_sgs(&gens, &base, 100, 10, 1000);
        let target = perm_f.compose(&perm_b).compose(&perm_u).compose(&perm_d);
        let valid_indices = crate::schreier::SchreierSims::get_stabilizing_color_gens(
            &"1;2;3;4;5;6;7;8".to_string(),
        );
        let fact =
            super::MinkwitzTable::factorize_minkwitz(&gens, &base, &tt, &target, &valid_indices);
        crate::testing_utils::TestingUtils::assert_index_path_equals_permutation(
            &fact,
            &target,
            &index_to_gen,
        );
    }
}
