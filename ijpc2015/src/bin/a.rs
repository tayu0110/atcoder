use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}
    println!("{}", a.iter().max().unwrap() + a.iter().sum::<usize>() + n);
}
