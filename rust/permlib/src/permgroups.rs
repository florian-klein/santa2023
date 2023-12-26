use crate::Permutation;

#[derive(Debug, Clone)]
pub struct GeneratingSet {
    pub generators: Vec<Permutation>,
}

impl GeneratingSet {
    pub fn new(generators: Vec<Permutation>) -> GeneratingSet {
        GeneratingSet { generators }
    }
}
