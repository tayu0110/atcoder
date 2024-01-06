use modint::{Mod1000000007, StaticModint};
use proconio::*;

type Mint = StaticModint<Mod1000000007>;

fn main() {
    input! {n: u32, m: u32, a: [u32; n]}

    let s = a.iter().sum::<u32>();

    if s > m {
        println!("0");
        return;
    }

    let mut res = Mint::one();
    for num in n + m - (s + n) + 1..=(n + m) {
        res *= Mint::raw(num);
    }

    for den in 1..=s + n {
        res /= Mint::raw(den);
    }

    println!("{}", res)
}
