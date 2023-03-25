use convolution_simd::convolution;
use modint::{Mod998244353, Modulo};
use proconio::input;

fn rec(now: usize, par: usize, t: &Vec<Vec<usize>>, memo: &mut Vec<Vec<Vec<u32>>>) {
    memo[now][0] = vec![1];
    memo[now][1] = vec![0, 1];

    if t[now].is_empty() {
        return;
    }

    for &to in &t[now] {
        if to == par {
            continue;
        }
        rec(to, now, t, memo);

        {
            let mut r1 = convolution::<Mod998244353>(memo[now][0].clone(), memo[to][0].clone());
            let mut r2 = convolution::<Mod998244353>(memo[now][0].clone(), memo[to][1].clone());
            if r1.len() < r2.len() {
                std::mem::swap(&mut r1, &mut r2);
            }
            r1.iter_mut().zip(r2).for_each(|(s, v)| {
                *s += v;
                if *s >= Mod998244353::MOD {
                    *s -= Mod998244353::MOD;
                }
            });
            memo[now][0] = r1;
        }
        {
            let mut r1 = convolution::<Mod998244353>(memo[now][1].clone(), memo[to][0].clone());
            let mut r2 = convolution::<Mod998244353>(memo[now][1].clone(), memo[to][1].clone());
            r2.remove(0);
            if r1.len() < r2.len() {
                std::mem::swap(&mut r1, &mut r2);
            }
            r1.iter_mut().zip(r2.into_iter()).for_each(|(s, v)| {
                *s += v;
                if *s >= Mod998244353::MOD {
                    *s -= Mod998244353::MOD;
                }
            });
            memo[now][1] = r1;
        }
    }
}

fn main() {
    input! {n: usize, p: [(usize, usize); n-1]}

    let mut t = vec![vec![]; n];
    for (a, b) in p {
        t[a - 1].push(b - 1);
        t[b - 1].push(a - 1);
    }

    let mut memo = vec![vec![vec![]; 2]; n];
    rec(0, 0, &t, &mut memo);

    memo[0][0].resize(n + 1, 0);
    memo[0][1].resize(n + 1, 0);
    for i in 1..=n {
        println!("{}", (memo[0][0][i] + memo[0][1][i]) % Mod998244353::MOD)
    }
}
