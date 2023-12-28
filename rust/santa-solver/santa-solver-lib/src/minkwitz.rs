use std::collections::HashMap;
#[derive(Debug)]
struct MinkwitzTable {
    table: HashMap<(usize, usize), String>,
}

impl MinkwitzTable {
    fn generate_table(&mut self, gen_to_str: &HashMap<usize, String>, max_word_size: usize) {}
}
