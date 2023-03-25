use proconio::*;

fn main() {
    input! {n: usize, k: usize, mut a: [usize; n]}
    a.sort();

    let mut t = vec![];
    for a in a {
        match t.last_mut() {
            Some((cnt, pa)) if *pa == a => *cnt += 1,
            _ => t.push((1, a)),
        }
    }

    t.sort();

    println!(
        "{}",
        t.iter()
            .take(t.len().saturating_sub(k))
            .map(|&(c, _)| c)
            .sum::<usize>()
    )
}
