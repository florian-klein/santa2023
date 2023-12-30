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

struct GroupGen {
    name: String,
    perm: Permutation,
}

struct GroupGens {
    elements: Vec<GroupGen>,
}

struct TransTable {
    table: HashMap<(usize, usize), PermAndWord>,
}

impl TransTable {
    pub fn new() -> Self {
        let table: HashMap<(usize, usize), PermAndWord> = HashMap::new();
        TransTable { table }
    }
}

impl PermAndWord {
    pub fn identity(n: usize) -> Self {
        PermAndWord {
            perm: Permutation::identity(n),
            word: Vec::new(),
        }
    }
}

impl MinkwitzTable {
    pub fn new() -> Self {
        let table: HashMap<(usize, usize), String> = HashMap::new();
        MinkwitzTable { table }
    }
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

    pub fn buildShortWordsSGS(
        gens: GroupGens,
        base: GroupBase,
        n: usize,
        s: usize,
        w: usize,
    ) -> TransTable {
        let mu_table = TransTable::new();
        // let permutation_size = gens.elements[0].perm.len();
        // for i in 0..base.elements.len() {
        //     mu_table.insert((i, base.elements[i]), PermAndWord::identity(n));
        // }
        // let mut max = n;
        // let mut limit = w;
        // let mut count = 0;
        // let group_iterator = PermutationGroupIterator::new(gens);
        // for (perm, word) in group_iterator {
        //     count += 1;
        //     if count >= max || is_table_full(gens, mu_table) {
        //         break;
        //     }
        //     let word = decode_index(i, level, gens.len);
        //     let pw = PermAndWord {
        //         perm: perm,
        //         word: word,
        //     };
        //     one_round(gens, base, limit, 0, mu_table, pw);
        //     if count % s == 0 {
        //         one_improve(gens, base, limit, mu_table);
        //         if !is_table_full(gens, mu_table) {
        //             fill_orbits(gens, base, limit, mu_table);
        //         }
        //         limit = limit * 5 / 4;
        //     }
        // }
        return mu_table;
    }

    pub fn is_table_full(n: usize, gens: GroupGens, mu_table: TransTable) -> bool {
        return false;
    }

    pub fn one_round(
        gens: GroupGens,
        base: GroupBase,
        limit: usize,
        c: usize,
        mu_table: TransTable,
        t: PermAndWord,
    ) {
        let mut i = c;
        // while i < base.elements.len() and t.perm.is_identity(){
        //     t = one_step(n, gens, base,i, t, mu_table);
        // }
    }
}
