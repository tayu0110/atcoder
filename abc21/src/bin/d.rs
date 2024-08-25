use proconio::*;
use static_modint::{combination, Mod1000000007};

fn main() {
    input! {n: u32, k: u32}

    let com = combination::<Mod1000000007>(n + k + 10);
    println!("{}", com(n + k - 1, k));
}
