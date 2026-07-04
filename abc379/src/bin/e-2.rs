use num::{BigUint, FromPrimitive, One, Zero};
use proconio::*;

fn main() {
    input! {n: usize, s: marker::Bytes}

    let mut res = BigUint::zero();
    for c in b'1'..=b'9' {
        let mut cum = vec![0; n];
        for i in s
            .iter()
            .enumerate()
            .filter_map(|(i, ch)| (*ch == c).then_some(i))
        {
            cum[n - 1 - i] += i + 1;
        }
        for i in (1..n).rev() {
            cum[i - 1] += cum[i];
        }
        let mut ten = BigUint::one();
        let c = BigUint::from_u8(c - b'0').unwrap();
        for i in 0..n {
            res += &c * BigUint::from_usize(cum[i]).unwrap() * &ten;
            ten *= BigUint::from_u8(10).unwrap();
        }
    }
    println!("{}", res);
}
