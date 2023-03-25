#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {s: Chars}

    let s = s.into_iter().map(|c| c.to_digit(10).unwrap() as i32).collect::<Vec<_>>();

    for i in 0..(1 << 3) {
        let mut sum = s[0];
        for j in 0..3 {
            if i & (1 << j) != 0 {
                sum += s[j+1];
            } else {
                sum -= s[j+1];
            }
        }

        if sum == 7 {
            print!("{}", s[0]);
            for j in 0..3 {
                if i & (1 << j) != 0 {
                    print!("+");
                } else {
                    print!("-");
                }
                print!("{}", s[j+1]);
            }

            println!("=7");
            std::process::exit(0);
        }
    }
}
