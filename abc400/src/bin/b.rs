use proconio::*;

fn main() {
    input! {n: usize, m: usize}

    let mut x = 0usize;
    let mut now = 1;
    for _ in 0..=m {
        x = x.saturating_add(now);
        now = now.saturating_mul(n);
    }

    if x > 1000_000_000 {
        println!("inf")
    } else {
        println!("{x}")
    }
}
