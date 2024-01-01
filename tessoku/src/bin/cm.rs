use proconio::*;
use std::collections::HashSet;

fn solve(a: &[usize]) -> HashSet<usize> {
    let len = a.len();

    let mut res = HashSet::new();
    for i in 0..1 << len {
        let sum = (0..len)
            .filter(|&j| i & (1 << j) != 0)
            .map(|j| a[j])
            .sum::<usize>();

        res.insert(sum);
    }

    res
}

fn main() {
    input! {n: usize, k: usize, a: [usize; n]}

    let r0 = solve(&a[..n / 2]);
    let r1 = solve(&a[n / 2..]);

    println!(
        "{}",
        (r0.contains(&k)
            || r1.contains(&k)
            || r0
                .into_iter()
                .filter(|&r0| r0 < k)
                .any(|r0| r1.contains(&(k - r0))))
        .then_some("Yes")
        .unwrap_or("No")
    )
}
