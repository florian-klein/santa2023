use log::debug;
use std::collections::HashMap;

use crate::permutation::Permutation;
#[derive(Debug)]

struct MinkwitzTable {
    table: HashMap<(usize, usize), String>,
}

struct GroupBase {
    elements: Vec<usize>,
}

struct PermAndWord {
    perm: Permutation,
    word: Vec<usize>,
}

struct GroupGens {}

struct TransTable {
    table: HashMap<(usize, usize), PermAndWord>,
}

impl TransTable {
    pub fn new() -> Self {
        let table: HashMap<(usize, usize), PermAndWord> = HashMap::new();
        TransTable { table }
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

    pub fn buildShortWordsSGS(gens: GroupGens, base: GroupBase, n: usize, s: usize, w: usize) {
        let mu_table = TransTable::new();
        for i in 0..base.elements.len() {
            // mu_table.insert((i, base.elements[i]),)
        }
    }
}
