#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {s: Chars, t: Chars}

    let mut is_match = vec![false; t.len() + 1];
    is_match[t.len()] = true;

    let mut f = true;
    for ((&cs, &ct), m) in s
        .iter()
        .rev()
        .zip(t.iter().rev())
        .zip(is_match.iter_mut().rev().skip(1))
    {
        if !f {
            *m = false;
            continue;
        }

        if cs == '?' || ct == '?' {
            *m = true;
            continue;
        }

        *m = cs == ct;
        f = cs == ct;
    }

    if is_match[0] {
        println!("Yes")
    } else {
        println!("No")
    }

    let mut f = true;
    for (i, (cs, ct)) in s.into_iter().zip(t.into_iter()).enumerate() {
        if !f {
            println!("No");
            continue;
        }

        if cs == '?' || ct == '?' {
            if is_match[i + 1] {
                println!("Yes")
            } else {
                println!("No")
            }
        } else {
            if cs == ct {
                if is_match[i + 1] {
                    println!("Yes")
                } else {
                    println!("No")
                }
            } else {
                f = false;
                println!("No");
            }
        }
    }
}
