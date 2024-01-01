use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, mut a: [u32; n], q: usize, x: [u32; q]}
    a.sort_unstable();

    let mut x = x
        .into_iter()
        .enumerate()
        .map(|(i, x)| (x, i as u32))
        .collect::<Vec<_>>();
    x.sort_unstable_by_key(|v| v.0);

    let mut res = vec![0u32; q];
    let mut now = 0;
    for (x, i) in x {
        while now < n && a[now] < x {
            now += 1;
        }

        res[i as usize] = now as u32;
    }

    println!("{}", res.iter().join("\n"))
}
