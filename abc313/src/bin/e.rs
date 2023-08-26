use proconio::*;
use static_modint::{Mod998244353, StaticModint};

type Modint = StaticModint<Mod998244353>;

fn main() {
    input! {_: usize, mut s: marker::Bytes}

    if s.windows(2).any(|v| v[0] != b'1' && v[1] != b'1') {
        println!("-1");
        return;
    }

    let mut prev = 1;
    let mut res = Modint::zero();
    while let Some(now) = s.pop() {
        if now == b'1' {
            res += Modint::one();
        } else {
            s.push(now);
            break;
        }
    }

    if s.is_empty() {
        println!("{}", res - Modint::one());
        return;
    }

    while let Some(now) = s.pop() {
        let now = (now - b'0') as u32;

        if s.is_empty() {
            if now == 1 {
                let inc = res * (Modint::raw(prev - 1));
                res += inc;
            }
            break;
        }

        if now == 1 {
            let inc = res * (Modint::raw(prev - 1)) + Modint::one();
            res += inc;
        } else {
            res += Modint::one();
        }
        prev = now;
        eprintln!("res: {res}");
    }

    println!("{}", res);
}
