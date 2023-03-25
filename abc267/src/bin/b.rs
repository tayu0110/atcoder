#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {s: Chars};

    if s[0] == '1' {
        println!("No");
        std::process::exit(0);
    }

    let b = [
        s[6] == '1',
        s[3] == '1',
        s[1] == '1' || s[7] == '1',
        s[0] == '1' || s[4] == '1',
        s[2] == '1' || s[8] == '1',
        s[5] == '1',
        s[9] == '1',
    ];

    for i in 0..7 {
        for j in i+1..7 {
            if !b[i] || !b[j] {
                continue;
            }
            let mut bad = false;
            let mut reach = false;
            for k in i+1..j {
                if b[k] {
                    bad = true;
                    break;
                }
                reach = true;
            }

            if !bad && reach {
                println!("Yes");
                std::process::exit(0);
            }
        }
    }

    println!("No");
}
