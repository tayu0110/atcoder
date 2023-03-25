use proconio::*;

fn main() {
    input! {n: usize, k: usize, mut l: [usize; n]}
    l.sort();
    l.reverse();

    println!("{}", l.into_iter().take(k).sum::<usize>())
}
