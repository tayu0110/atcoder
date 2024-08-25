use proconio::*;

fn main() {
    input! {a: [usize; 4], b: [usize; 4]}
    println!("{}", a.iter().sum::<usize>().max(b.iter().sum()))
}
