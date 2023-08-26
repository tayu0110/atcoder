use itertools::Itertools;
use proconio::*;

fn main() {
    input! {v: usize, e: usize, k: u32, e: [(usize, usize); e]}

    if e.is_empty() {
        println!("{}", (0..k).join("\n"));
        return;
    }

    let mut bad = (0..v).map(|v| 1 << v).collect::<Vec<_>>();
    for (a, b) in e {
        bad[a] |= 1 << b;
        bad[b] |= 1 << a;
    }

    for i in 0..v {
        println!("{:05b}", bad[i]);
    }

    for i in 0u32..1 << v {
        if i.count_ones() != k {
            continue;
        }

        if (0..v)
            .filter(|&v| i & (1 << v) != 0)
            .any(|v| (bad[v] & i).count_ones() > 1)
        {
            continue;
        }

        println!("{}", (0..v).filter(|&v| (1 << v) & i != 0).join("\n"));
        return;
    }

    unreachable!()
}
