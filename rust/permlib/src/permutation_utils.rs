use crate::Permutation;
// WARN: DO NOT USE
pub fn cycle_str_to_permutation(cycle_str: &str) -> Permutation {
    let mut elements = vec![0; cycle_str.len()];

    let cycle: Vec<&str> = cycle_str.split_whitespace().collect();
    let cycle: Vec<usize> = cycle.iter().map(|&x| x.parse::<usize>().unwrap()).collect();

    for i in 0..cycle.len() {
        elements[cycle[i]] = cycle[(i + 1) % cycle.len()];
    }

    Permutation::new(elements)
}

pub fn array_string_to_permutation(array_string: &str) -> Permutation {
    let mut elements = vec![0; array_string.len()];

    let array: Vec<&str> = array_string.split_whitespace().collect();
    let array: Vec<usize> = array.iter().map(|&x| x.parse::<usize>().unwrap()).collect();

    for i in 0..array.len() {
        elements[i] = array[i];
    }

    Permutation::new(elements)
}
