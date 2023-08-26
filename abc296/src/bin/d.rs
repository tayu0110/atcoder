use num::integer::Roots;
use proconio::*;

fn main() {
    input! {n: usize, m: usize}

    if n * n < m {
        println!("-1");
        return;
    }

    if n >= m {
        println!("{}", m);
        return;
    }

    let sq = m.sqrt();
    let dir = (sq + 1) * (sq + 1) - m;
    let mut t = vec![1; dir + 1];
    for i in 2..=sq + 1 {
        let start = (m + i - 1) / i * i;
        for j in (start - m..=dir).step_by(i) {
            t[j] = t[j].max(i);
        }
    }

    for i in 0..=dir {
        if t[i] <= n && (i + m) / t[i] <= n {
            println!("{}", i + m);
            return;
        }
    }

    println!("-1")
}
