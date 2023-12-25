use crate::Permutation;
pub struct GeneratingSet {
    pub generators: Vec<Permutation>,
}

impl GeneratingSet {
    pub fn new(generators: Vec<Permutation>) -> GeneratingSet {
        GeneratingSet { generators }
    }
}
