#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {mut s: Chars}

    let mut buf = vec![];
    let mut ns = s[0].to_string();
    for v in s.windows(2) {
        if v[0].is_ascii_uppercase() && v[1].is_ascii_uppercase() {
            if ns.len() == 1 {
                ns.push(v[1]);
                buf.push(ns);
                ns = "".to_string();
            } else {
                buf.push(ns);
                ns = v[1].to_string()
            }
        } else {
            ns.push(v[1]);
        }
    }
    buf.push(ns);

    buf.sort_by_key(|s| s.to_ascii_lowercase());
    println!("{}", buf.into_iter().collect::<String>());
}
