use proconio::*;
use static_modint::{combination, Mod1000000007};

fn main() {
    input! {x: u32, y: u32}

    let com = combination::<Mod1000000007>(x + y);
    println!("{}", com(x + y, x))
}
