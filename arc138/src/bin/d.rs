use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, k: u32}

    if k > 1 && n == k as usize {
        println!("No");
        return;
    }

    if k % 2 == 0 {
        println!("No");
        return;
    }

    let gray = (0..1 << n).map(|v| v ^ (v >> 1)).collect::<Vec<_>>();

    let mut base = vec![];
    let mut a = vec![];
    for i in 0u32..1 << n {
        if i.count_ones() != k {
            continue;
        }

        let mut v = i;
        for &b in &base {
            v = v.min(v ^ b);
        }

        if v > 0 {
            base.push(v);
            a.push(i);
        }
    }

    let mut res = vec![];
    for g in gray {
        let mut r = 0;
        for i in 0..n {
            if g & (1 << i) != 0 {
                r ^= a[i];
            }
        }

        res.push(r);
    }

    println!("Yes");
    println!("{}", res.into_iter().join(" "))
}
