#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {s: Chars}

    if s[0] == s[1] && s[1] == s[2] && s[2] == s[3] {
        println!("Weak");
        std::process::exit(0);
    }

    for v in s.windows(2) {
        let (c, nc) = (v[0], v[1]);

        let (c, nc) = (c as u8 - b'0', nc as u8 - b'0');
        if (c + 1) % 10 != nc {
            println!("Strong");
            std::process::exit(0);
        }
    }

    println!("Weak");
}
