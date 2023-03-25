#[allow(unused_imports)]
use proconio::{input, marker::Chars, marker::Bytes, source::line::LineSource};

fn main() {
    input! {mut s: Bytes, t: Bytes}

    let mut res = 0;
    for i in 0..s.len() {
        if i+1 < s.len() && s[i] != t[i] {
            s.swap(i, i+1);
            res += 1;
        }

        if s[i] != t[i] {
            println!("No");
            std::process::exit(0);
        }
    }

    if res <= 1 {
        println!("Yes");
    } else {
        println!("No");
    }
}
