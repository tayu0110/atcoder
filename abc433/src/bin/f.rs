use montgomery_modint::Mod998244353;
use proconio::*;
use static_modint::combination;

fn main() {
    input! {s: marker::Bytes}

    let com = combination::<Mod998244353>(s.len() as u32 + 10);
}
