use montgomery_modint::{Mod998244353, Modulo, MontgomeryModint};
use polynomial::Polynomial;
use proconio::*;

fn main() {
    input! {n: usize, m: usize, a: [i32; n + 1], c: [usize; n + m + 1]}

    let a = a
        .into_iter()
        .map(|a| MontgomeryModint::<Mod998244353>::new((a + Mod998244353::N as i32) as u32))
        .collect::<Vec<_>>();
}
