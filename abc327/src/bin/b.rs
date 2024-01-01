use proconio::*;

fn main() {
    input! {b: u64}

    for i in 1..60 {
        let mut now = 1u64;
        for _ in 0..i {
            now = now.saturating_mul(i);
        }

        if now == b {
            println!("{i}");
            return;
        }
    }

    println!("-1")
}
