use proconio::*;
use static_modint::{combination, Mod1000000007};

fn main() {
    input! {n: u32, r: u32}

    let com = combination::<Mod1000000007>(1000010);
    println!("{}", com(n, r))
}
