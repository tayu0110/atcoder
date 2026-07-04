use proconio::*;

fn main() {
    input! {n: usize, mut s: String}
    while s.len() < n {
        s.insert(0, 'o');
    }
    println!("{}", s)
}
