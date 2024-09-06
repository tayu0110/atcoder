use proconio::*;

fn main() {
    input! {n: usize, a: [i32; n]}

    let b = a.windows(2).map(|v| v[1] - v[0]).collect::<Vec<_>>();
    let mut t = vec![];
    for b in b {
        match t.last_mut() {
            Some((p, c)) if *p == b => *c += 1usize,
            _ => t.push((b, 1)),
        }
    }

    println!(
        "{}",
        t.into_iter()
            .map(|(_, c)| c * (c - 1) / 2 + c)
            .sum::<usize>()
            + n
    )
}
