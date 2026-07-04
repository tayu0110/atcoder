use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, a: [i32; n]}

    for v in (1..=n as i32).permutations(n) {
        if v.iter().zip(&a).filter(|v| v.1 != &-1).all(|(v, a)| v == a) {
            println!("Yes");
            println!("{}", v.iter().join(" "));
            return;
        }
    }
    println!("No")
}
