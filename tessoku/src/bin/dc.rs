use proconio::*;
use static_modint::{combination, Mod1000000007};

fn main() {
    input! {h: u32, w: u32}
    let com = combination::<Mod1000000007>(1000000);
    println!("{}", com(h + w - 2, h - 1))
}
