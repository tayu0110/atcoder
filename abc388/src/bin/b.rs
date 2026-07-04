use proconio::*;

fn main() {
    input! {n: usize, d: usize, p: [(usize, usize); n]}

    for k in 1..=d {
        let mut max = 0;
        for &(t, l) in &p {
            max = max.max((l + k) * t);
        }
        println!("{max}")
    }
}
