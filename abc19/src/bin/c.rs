use proconio::*;

fn main() {
    input! {n: usize, mut a: [usize; n]}
    a.iter_mut().for_each(|a| *a >>= a.trailing_zeros());
    a.sort_unstable();
    a.dedup();

    println!("{}", a.len());
}
