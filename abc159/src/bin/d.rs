use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut t = vec![0usize; n + 1];
    for &a in &a {
        t[a] += 1;
    }
    let res = t
        .iter()
        .map(|&v| v * v.saturating_sub(1) / 2)
        .sum::<usize>();

    a.into_iter()
        .map(|a| res + 1 - t[a])
        .inspect(|&a| println!("{}", a))
        .count();
}
