use proconio::*;

fn main() {
    input! {n: usize, p: [usize; n]}

    println!("{}", p.iter().sum::<usize>() - p.iter().max().unwrap() / 2);
}
