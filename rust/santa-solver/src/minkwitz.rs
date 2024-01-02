use crate::groups::PermutationGroupIterator;
use log::debug;
use std::collections::HashMap;

use crate::permutation::Permutation;
#[derive(Debug)]

struct MinkwitzTable {
    table: HashMap<(usize, usize), String>,
}

#[derive(Clone, Debug)]
struct GroupBase {
    elements: Vec<usize>,
}

#[derive(Clone, Debug)]
struct PermAndWord {
    perm: Permutation,
    word: Vec<usize>,
    pub news: bool,
}

#[derive(Debug)]
struct GroupGen {
    name: String,
    perm: Permutation,
}

#[derive(Debug)]
struct GroupGens {
    elements: Vec<GroupGen>,
}

#[derive(Debug)]
struct TransTable {
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

impl PermAndWord {
    pub fn new(perm: Permutation, word: Vec<usize>) -> Self {
        PermAndWord {
            perm,
            word,
            news: true,
        }
    }
    pub fn get_inverse(&self) -> Self {
        let mut inverse_word = self.word.clone();
        inverse_word.reverse();
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
        let mut new_word = self.word.clone();
        new_word.extend(&other.word);
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
}

impl MinkwitzTable {
    pub fn factorize_minkwitz(
        gens: GroupGens,
        base: GroupBase,
        nu: TransTable,
        target: Permutation,
    ) -> Vec<usize> {
        let mut list: Vec<usize> = Vec::new();
        let mut perm = target.clone();
        for i in 0..base.elements.len() {
            let omega = perm.p[base.elements[i]];
            let table_entry = nu.table.get(&(i, omega)).unwrap();
            perm = table_entry.perm.compose(&perm);
            let new_word = &table_entry.word;
            list.extend(new_word);
        }
        if perm != Permutation::identity(target.len()) {
            debug!("We could not find a factorization!");
            return Vec::new();
        }
        // todo: list.inverse()
        debug!("Todo: list.inverse(gens)");
        return list;
    }

    /* Options:
    n: max number of rounds
    s: reset each s rounds
    w: limit for word size
     */
    pub fn build_short_word_sgs(
        gens: GroupGens,
        base: GroupBase,
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
            if count % s == 0 {
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
        let t_inv = t.get_inverse();
        let mut result = PermAndWord::identity(gens.elements[0].perm.len());
        if let Some(table_entry) = mu_table.get(&(i, j)) {
            result = t_inv.compose(&table_entry);
            if t.word.len() < table_entry.word.len() {
                println!("Assigning {:?} to {:?}", (i, j), t_inv.clone());
                mu_table.insert((i, j), t_inv.clone());
                drop(Self::one_step(gens, base, i, &t_inv, mu_table));
            }
        } else {
            println!(
                "Assigning {:?} to {:?} doesnt exist before",
                (i, j),
                t_inv.clone()
            );
            mu_table.insert((i, j), t_inv.clone());
            drop(Self::one_step(gens, base, i, &t_inv, mu_table));
            result = PermAndWord::identity(gens.elements[0].perm.len());
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
        println!("gens: {:?}", gens);
        println!("base: {:?}", base);
        println!("limit: {:?}", limit);
        println!("c: {:?}", c);
        println!("t value {:?}", t);
        while i < base.elements.len() && !t.perm.is_identity() && t_new.word.len() < limit {
            println!("Running one step!");
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
        let t = PermAndWord::identity(gens.elements[0].perm.len());
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
                            let new_perm = x_elm.compose(&y_elm);
                            Self::one_round(gens, base, limit, j, mu_table, new_perm);
                        }
                    }
                }
            }
            for x in 0..base.elements.len() {
                let x_elm = mu_table.get(&(j, x));
                if x_elm.is_some() {
                    let pw = x_elm.unwrap();
                    // todo: fix this
                    // pw.set_news(false);
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
                    let j = table_entry.perm.p[base.elements[i]];
                    if !orbit.contains(&j) {
                        orbit.push(j);
                    }
                }
            }
            for j in i + 1..base.elements.len() {
                for i in 0..base.elements.len() {
                    let x = mu_table.get(&(j, i));
                    if !x.is_some() {
                        continue;
                    }
                    let x1 = x.unwrap().get_inverse();
                    let orbit_x: Vec<usize> =
                        orbit.iter().map(|it| x.unwrap().perm.p[*it]).collect();
                    let new_pts: Vec<usize> = orbit_x
                        .iter()
                        .filter(|it| !orbit.contains(it))
                        .map(|it| *it)
                        .collect();
                    for p in new_pts {
                        if let Some(table_entry) = mu_table.get(&(i, p)) {
                            let t1 = x1.compose(table_entry);
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
    use crate::permutation::Permutation;

    fn is_valid_sgs(tt: super::TransTable, base: super::GroupBase) {
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
                        if p.p[base.elements[k]] != base.elements[k] {
                            result = false;
                            println!("Table entry {:?} is not valid due to base elements", (i, j));
                        }
                    }
                    if p.p[j] != base.elements[i] {
                        println!("Table entry {:?} is not valid", (i, j));
                        result = false;
                    }
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
        let perm1 = Permutation::parse_permutation_from_cycle("(1,2)", 3);
        let perm2 = Permutation::parse_permutation_from_cycle("(2,3)", 3);
        let gen1 = super::GroupGen::new("a".to_string(), perm1);
        let gen2 = super::GroupGen::new("b".to_string(), perm2);
        let gens = super::GroupGens::new(vec![gen1, gen2]);
        // 2) Create Base
        let base = super::GroupBase {
            elements: vec![0, 1, 2],
        };
        let tt = MinkwitzTable::build_short_word_sgs(gens, base.clone(), 100, 1000, 1000);
        println!("{:?}", tt);
        is_valid_sgs(tt, base);
    }
}
