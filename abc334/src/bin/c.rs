use std::collections::HashSet;

use proconio::*;

fn main() {
    input! {n: usize, k: usize, mut a: [usize; k]}

    let mut a = {
        let mut b = vec![];
        let a = a.into_iter().collect::<HashSet<_>>();
        for i in 1..=n {
            b.push(i);
            if !a.contains(&i) {
                b.push(i);
            }
        }
        b
    };

    if a.len() % 2 == 0 {
        println!("{}", a.chunks_exact(2).map(|v| v[1] - v[0]).sum::<usize>());
        return;
    }

    let mut f = vec![0; a.len()];
    let mut b = vec![0; a.len()];
    for (i, v) in a.chunks_exact(2).enumerate() {
        if i > 0 {
            f[i * 2] = f[i * 2 - 1];
        }
        f[i * 2] += v[1] - v[0];
        f[i * 2 + 1] = f[i * 2];
    }
    a.reverse();
    for (i, v) in a.chunks_exact(2).enumerate() {
        if i > 0 {
            b[i * 2] = b[i * 2 - 1];
        }
        b[i * 2] += v[0] - v[1];
        b[i * 2 + 1] = b[i * 2];
    }
    b.reverse();

    let mut res = usize::MAX;
    for i in 0..a.len() {
        let mut sum = 0;
        if i > 0 {
            sum += f[i - 1];
        }
        if i + 1 < a.len() {
            sum += b[i + 1];
        }
        res = res.min(sum);
    }

    println!("{res}")
}
