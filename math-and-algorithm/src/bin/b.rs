use proconio::*;

fn main() {
    input! {a: [usize; 3]}
    println!("{}", a.iter().sum::<usize>())
}
