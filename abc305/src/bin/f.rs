use std::io::Write;

use proconio::*;

fn main() {
    let mut stdin = source::line::LineSource::new(std::io::BufReader::new(std::io::stdin()));
    macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut stdin, $($tt)*)));

    input! {n: usize, _m: usize}

    let mut par = vec![std::usize::MAX; n];
    let mut now = 0;

    'base: for _ in 0..2 * n {
        input! {k: usize, v: [usize; k]}

        if v.contains(&n) {
            println!("{}", n);
            std::io::stdout().flush().unwrap();
            input! {s: String};

            if s == "OK" {
                return;
            }
        }

        for to in v.iter().map(|&to| to - 1) {
            if par[to] != std::usize::MAX {
                continue;
            }

            par[to] = now;
            now = to;
            println!("{}", to + 1);
            std::io::stdout().flush().unwrap();
            continue 'base;
        }

        println!("{}", par[now] + 1);
        now = par[now];
        std::io::stdout().flush().unwrap();
    }

    unreachable!()
}
