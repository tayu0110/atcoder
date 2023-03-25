#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {a: usize, b: usize}

    let mut res = 0;
    for now in a..=b {
        let s = now.to_string();
        let mut t = s.clone().chars().collect::<Vec<_>>();
        t.reverse();
        let t = t.into_iter().collect::<String>();

        if s == t {
            res += 1;
        }
    }

    println!("{}", res);
}
