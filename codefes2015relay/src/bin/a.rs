use proconio::*;

fn main() {
    input! {t: usize}
    let mut s = vec![];
    for i in 0..10 {
        if i % 2 == 0 {
            s.extend(i * 20 + 1..=i * 20 + 20);
        } else {
            s.extend((i * 20 + 1..=i * 20 + 20).rev());
        }
    }
    println!(
        "{}",
        s.iter()
            .enumerate()
            .filter(|s| s.0 % 20 == t - 1)
            .map(|s| s.1)
            .sum::<usize>()
    );
}
