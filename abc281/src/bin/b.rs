#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {mut s: Chars}

    let len = s.len();

    if len != 8 {
        println!("No");
        return;
    }

    if !s[0].is_ascii_alphabetic() {
        println!("No");
        return;
    }

    if !s[len - 1].is_ascii_alphabetic() {
        println!("No");
        return;
    }

    s.pop().unwrap();
    s.remove(0);

    if s.len() != 6 {
        println!("No");
        return;
    }

    if let Ok(s) = s.into_iter().collect::<String>().parse::<usize>() {
        if (100000..1000000).contains(&s) {
            println!("Yes");
        } else {
            println!("No");
        }
    } else {
        println!("No");
    }
}
