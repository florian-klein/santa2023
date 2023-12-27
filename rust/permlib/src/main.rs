mod factorization;
mod minkwitz;
mod permgroups;
mod permutation;
mod permutation_utils;
mod schreier;
mod word_length_iter;
use permutation::Permutation;

fn main() {
    let perm1 = Permutation::new(vec![2, 0, 1]);
    println!("{:?}", perm1);
    println!("{:?}", perm1.inverse());
    // println!("{:?}", permutation_utils::cycle_str_to_permutation("(1 2 3)"));
}
