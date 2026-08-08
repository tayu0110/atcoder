use proconio::*;

fn main() {
    input! {m: usize, d: usize, s: marker::Bytes}

    println!(
        "{}",
        (0..m)
            .filter(|i| {
                s[i.saturating_sub(d)..=(i + d).min(m - 1)]
                    .iter()
                    .all(|b| *b == b'.')
            })
            .count()
    )
}
