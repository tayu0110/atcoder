use proconio::*;

fn main() {
    input! {n: usize, m: usize, e: [(usize, i32); n]}

    for k in 1..=m {
        println!(
            "{}",
            e.iter()
                .copied()
                .filter_map(|e| (e.0 == k).then_some(e.1))
                .max()
                .unwrap_or(-1)
        );
    }
}
