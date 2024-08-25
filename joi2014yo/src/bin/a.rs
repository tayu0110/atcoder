use proconio::*;

fn main() {
    input! {a: [usize; 5]}

    println!("{}", a.into_iter().map(|a| a.max(40)).sum::<usize>() / 5)
}
