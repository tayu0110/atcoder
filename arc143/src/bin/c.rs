#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
	input! {n: usize, x: usize, y: usize, a: [usize; n]};

    if x == y {
        let sum = a.iter().fold(0usize, |sum, x| sum + *x / y);
        if sum % 2 == 0 {
            println!("First");
        } else {
            println!("Second");
        }
    } else if x < y {
        let mut good = false;
        for v in a {
            if x <= v % (x + y) && v % (x + y) < y {
                good = true;
            }
        }

        if good {
            println!("First");
        } else {
            println!("Second");
        }
    } else {
        let mut good = false;
        for v in a {
            if y <= v % (x + y) && v & (x + y) < x {
                good = true;
            }
        }

        if good {
            println!("Second");
        } else {
            println!("First");
        }
    }
}
