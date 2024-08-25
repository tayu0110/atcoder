use num::{BigUint, FromPrimitive};
use proconio::*;

fn main() {
    input! {_: usize}

    let mut f = BigUint::from_u32(5).unwrap().pow(50);
    let mut res = f.to_string();
    for _ in 0..50 {
        f *= 5u32;
        res.push_str(f.to_string().as_str());
    }

    println!("{res}")
}
