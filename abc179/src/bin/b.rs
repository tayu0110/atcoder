use proconio::*;

fn main() {
    input! {n: usize, d: [(usize, usize); n]}
    if d.windows(3).any(|v| v.iter().all(|v| v.0 == v.1)) {
        println!("Yes")
    } else {
        println!("No")
    }
}
