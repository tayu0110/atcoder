#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, p: [i32; n]};

    let mut plus = vec![0; n+1];
    let mut minus = vec![0; n+1];

    let mut res = 0usize;
    for (i, v) in p.into_iter().enumerate() {
        let diff = (i as i32 - v + n as i32) % n as i32;
        let ndiff = n as i32 - diff;

        plus[diff as usize] += 1;
        minus[ndiff as usize] += 1;

        res += std::cmp::min(diff as usize, ndiff as usize);
    }
    if res == 0 {
        println!("0");
        std::process::exit(0);
    }

    let (mut np, mut nm) = ((n-1) / 2, 1);
    let (mut sp, mut sm) = (0, 0);
    for (i, (tp, tm)) in plus.iter().zip(minus.iter()).take(n/2+1).enumerate() {
        if i == 0 {
            sp += *tp;
        } else if i == n/2 && n & 1 == 0 {
            sm += *tm;
        } else {
            sp += *tp;
            sm += *tm;
        }
    }

    let mut sum = res;
    for _ in 0..n {
        res = std::cmp::min(res, sum);
        if res == 0 {
            println!("0");
            std::process::exit(0);
        }
        sum = sum + sp - sm;
        if n & 1 == 1 {
            sum -= plus[np];
        }
        sp = sp + minus[nm] - plus[np];
        sm = sm + plus[np] - minus[nm];
        nm += 1;
        np = (np + n - 1) % n;
    }

    println!("{}", res);
}
