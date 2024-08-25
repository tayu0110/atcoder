use num::{BigUint, FromPrimitive};
use proconio::*;

fn main() {
    input! {n: usize, mut a: usize, k: String, b: [usize; n]}

    let mut buf = vec![];
    let mut seen = vec![false; n];
    while !seen[a - 1] {
        buf.push(a);
        seen[a - 1] = true;
        a = b[a - 1];
    }

    let mut k = k.parse::<BigUint>().unwrap();

    let pos = buf.iter().position(|&b| b == a).unwrap();
    if k < BigUint::from_usize(pos).unwrap() {
        let p: usize = k.try_into().unwrap();
        println!("{}", buf[p]);
        return;
    }

    k -= pos;
    k %= buf.len() - pos;

    let k: usize = k.try_into().unwrap();
    println!("{}", buf[pos + k])
}
