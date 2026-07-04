use proconio::*;

fn main() {
    input! {_: usize, m: usize, s: marker::Bytes, t: marker::Bytes}
    println!(
        "{}",
        s.windows(m)
            .map(|s| s
                .iter()
                .copied()
                .zip(t.iter().copied())
                .map(|(s, t)| (s + 10 - t) as usize % 10)
                .sum::<usize>())
            .min()
            .unwrap()
    )
}
