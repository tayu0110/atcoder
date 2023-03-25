#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {_n: usize, s: Chars};

    let mut nt = std::collections::VecDeque::new();
    for v in s {
        nt.push_back(v);
    }

    let mut bflag = false;
    while nt.len() > 1 {
        let f = nt.pop_front().unwrap();
        let b = nt.pop_back().unwrap();
        if f == 'B' && b == 'A' {
            if nt.is_empty() {
                println!("No");
                std::process::exit(0);
            }
            println!("Yes");
            std::process::exit(0);
        } else if f == 'A' && b == 'B' {
            if bflag {
                println!("Yes");
                std::process::exit(0);
            }
            println!("No");
            std::process::exit(0);
        } else if f == 'B' && b == 'B' {
            println!("Yes");
            std::process::exit(0);
        } else {
            // f == 'A' && b == 'A'
            if bflag {
                println!("Yes");
                std::process::exit(0);
            }
            bflag = true;
        }
    }

    println!("Yes");
}
