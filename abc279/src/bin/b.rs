#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {s: Chars, t: Chars}

    let slen = s.len();
    let tlen = t.len();

    if tlen > slen {
        println!("No");
        return;
    }

    for i in 0..slen {
        let mut bad = false;
        for (j, k) in (0..tlen).into_iter().zip(i..) {
            if k == slen {
                bad = true;
                break;
            }
            if t[j] != s[k] {
                bad = true;
                break;
            }
        }

        if !bad {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
