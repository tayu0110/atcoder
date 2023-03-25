#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {a: i64, b: i64, c: i64, d: i64}

    if a == c && b == d {
        println!("0");
    } else if a + b == c + d || a - b == c - d || (a-c).abs() + (b-d).abs() <= 3 {
        println!("1");
    } else {
        for dy in -3..=3 {
            for dx in -3..=3 {
                let na = a + dy;
                let nb = b + dx;
                if (a-na).abs() + (b-nb).abs() <= 3 {
                    if na + nb == c + d || na - nb == c - d || (na-c).abs() + (nb-d).abs() <= 3 {
                        println!("2");
                        std::process::exit(0);
                    }
                }
            }
        }

        if (a + b) % 2 == (c + d) % 2 {
            println!("2");
            std::process::exit(0);
        }

        println!("3");
    }
}
