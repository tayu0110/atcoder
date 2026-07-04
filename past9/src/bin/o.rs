use polynomial::Polynomial;
use proconio::*;
use static_modint::{Mod998244353, Modulo};

const N: usize = 100001;
const MUL: [u32; N] = {
    let mut buf = [0; N];
    buf[0] = 1;
    buf[1] = 1;
    let mut i = 2;
    while i < N {
        buf[i] = (buf[i - 1] as u64 * i as u64 % Mod998244353::N as u64) as u32;
        i += 1;
    }
    buf
};

fn main() {
    input! {n: usize, b: [u32; n], s: [u32; n]}

    let mut cnt = vec![0u32; N];
    for b in b {
        cnt[b as usize] += 1;
    }

    let a = Polynomial::<Mod998244353>::from(cnt.clone());
    let b = cnt.into_iter().rev().collect::<Polynomial<Mod998244353>>();
    let res: Vec<u32> = (a * b).into();
    let mut sum = 0;
    for i in N..N + n {
        sum += (res[i] as u64 * MUL[n - 2] as u64 % Mod998244353::N as u64 * s[i - N] as u64
            % Mod998244353::N as u64
            * (n - (i - N) - 1) as u64
            % Mod998244353::N as u64) as u32;
        sum %= Mod998244353::N;
    }
    println!("{sum}")
}
