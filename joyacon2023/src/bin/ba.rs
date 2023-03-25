use itertools::Itertools;
use permutohedron::LexicalPermutation;
use proconio::input;

fn main() {
    input! {n: usize, mut p: [usize; n]}

    p.prev_permutation();
    println!("{}", p.iter().join(" "))
}
