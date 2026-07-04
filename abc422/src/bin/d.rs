use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, k: usize}

    let t = 1 << n;
    let mut res = vec![k / t; t];
    let rem = k % t;
    for i in 0..rem {
        let mut j = 0;
        for k in 0..n {
            j <<= 1;
            j |= (i >> k) & 1;
        }

        res[j] += 1;
    }

    let mut a = res.clone();
    let mut x = 0usize;
    for _ in 0..n {
        x = x.max(a.iter().max().unwrap() - a.iter().min().unwrap());
        a = a.chunks_exact(2).map(|v| v[0] + v[1]).collect();
    }

    println!("{x}");
    println!("{}", res.iter().join(" "))
}
