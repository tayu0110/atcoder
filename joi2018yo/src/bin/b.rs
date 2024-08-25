use proconio::*;

fn main() {
    input! {n: usize, a: [u8; n]}

    let mut t = Vec::with_capacity(n);
    for a in a {
        match t.last_mut() {
            Some((pa, cnt)) if *pa == a => *cnt += 1,
            _ => t.push((a, 1)),
        }
    }

    println!(
        "{}",
        t.into_iter()
            .filter_map(|v| (v.0 == 1).then_some(v.1))
            .max()
            .unwrap_or(0)
            + 1
    )
}
