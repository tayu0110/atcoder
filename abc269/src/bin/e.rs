use std::io::Write;

#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    let mut stdin = LineSource::new(std::io::BufReader::new(std::io::stdin()));
    macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut stdin, $($tt)*)));

    input! {n: usize}

    let (mut l, mut r) = (1, n+1);
    while r -l > 1 {
        let m = (r + l) / 2;
        println!("? {} {} {} {}", l, m, 1, n);
        std::io::stdout().flush().unwrap();
        input! {t: i32}

        if t < 0 {
            std::process::exit(1);
        }

        if t as usize == m - l + 1 {
            l = m+1;
        } else {
            r = m;
        }
    }

    println!("? {} {} {} {}", l, l, 1, n);
    std::io::stdout().flush().unwrap();
    input! {t: i32}
    if t < 0 {
        std::process::exit(1);
    }
    let row = if t == 1 {
        r
    } else {
        l
    };

    let (mut l, mut r) = (1, n+1);
    while r - l > 1 {
        let m = (r + l) / 2;
        println!("? {} {} {} {}", 1, n, l, m);
        std::io::stdout().flush().unwrap();
        input! {t: i32}

        if t < 0 {
            std::process::exit(0);
        }

        if t as usize == m - l + 1 {
            l = m+1;
        } else {
            r = m;
        }
    }

    println!("? {} {} {} {}", 1, n, l, l);

    input! {t: i32}
    if t < 0 {
        std::process::exit(1);
    }
    let column = if t == 1 {
        r
    } else {
        l
    };

    println!("! {} {}", row, column);
}

