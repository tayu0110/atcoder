#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {s: Chars}

    let mut t = vec![];
    for _ in 0..1000 {
        t.append(&mut vec!['o', 'x', 'x']);
    }

    for v in t.windows(s.len()) {
        let mut bad = false;
        for (c, nc) in s.iter().zip(v.iter()) {
            if c != nc {
                bad = true;
                break;
            }
        }

        if !bad {
            println!("Yes");
            std::process::exit(0);
        }
    }

    println!("No");
}
