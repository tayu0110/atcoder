#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {x: Chars, m: u128}

    if x.len() == 1 {
        let x = x[0] as u8 - b'0';
        if (x as u128) <= m {
            println!("1");
        } else {
            println!("0");
        }

        std::process::exit(0);
    }

    let check = |mut x: Vec<char>, mid: u128| {
        let mut nx = 0;
        let mut b = 1u128;

        while let Some(c) = x.pop() {
            if b > m {
                x.push(c);
                break;
            }
            let c = c.to_digit(10).unwrap() as u128;
            nx += c * b;
            if nx > m {
                break;
            }
            b *= mid;
        }

        nx <= m && x.is_empty()
    };

    let d = (*x.iter().max().unwrap() as u8 - b'0') as u128;
    let (mut l, mut r) = (d, m+1);
    while r - l > 1 {
        let mid = (r + l) / 2;

        if check(x.clone(), mid) {
            l = mid;
        } else {
            r = mid;
        }
    }

    println!("{}", l - d);
}
