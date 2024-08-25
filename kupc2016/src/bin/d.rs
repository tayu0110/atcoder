use proconio::source::line::LineSource;
use proconio::*;
use std::io::BufReader;
use std::io::{self, Write};

fn out(s: &[char], t: &[char]) {
    println!("{}", s.iter().collect::<String>());
    println!("{}", t.iter().collect::<String>());
    std::io::stdout().flush().unwrap();
}

fn out_front(t: &[char], c: char) {
    println!("{}{}", c, t.iter().collect::<String>());
    std::io::stdout().flush().unwrap();
}

fn out_back(t: &[char], c: char) {
    println!("{}{}", t.iter().collect::<String>(), c);
    std::io::stdout().flush().unwrap();
}

fn main() {
    let mut stdin = LineSource::new(BufReader::new(io::stdin()));
    macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut stdin, $($tt)*)));

    input! {n: usize}
    let mut res = vec![vec![]; 2];

    'base: for _ in 0..n {
        for u in ['.', '#'] {
            for d in ['.', '#'] {
                out_back(&res[0], u);
                out_back(&res[1], d);

                input! {f: String}
                if f == "T" {
                    res[0].push(u);
                    res[1].push(d);
                    continue 'base;
                } else if f == "end" {
                    res[0].push(u);
                    res[1].push(d);
                    out(&res[0], &res[1]);
                    return;
                }
            }
        }

        break;
    }

    'base: for _ in 0..n - res[0].len() {
        for u in ['.', '#'] {
            for d in ['.', '#'] {
                out_front(&res[0], u);
                out_front(&res[1], d);

                input! {f: String}
                if f == "T" {
                    res[0].insert(0, u);
                    res[1].insert(0, d);
                    continue 'base;
                } else if f == "end" {
                    res[0].insert(0, u);
                    res[1].insert(0, d);
                    out(&res[0], &res[1]);
                    return;
                }
            }
        }

        break;
    }

    out(&res[0], &res[1]);

    input! {f: String}

    assert_eq!(f, "end");
}
