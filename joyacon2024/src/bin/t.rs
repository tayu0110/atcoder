use proconio::*;

fn main() {
    input! {n: usize, p: [(usize, usize); n]}

    let &(a, b) = p.iter().max_by_key(|v| v.0).unwrap();
    println!("{}", a + b);
}
