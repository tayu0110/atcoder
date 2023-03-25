use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, m: usize};

    for v in (1..m+1).combinations(n) {
        let prev = 0;
        let mut bad = false;
        for k in &v {
            if prev >= *k {
                bad = true;
                break;
            }
        }
        if bad {
            continue;
        }

        for (i, v) in v.into_iter().enumerate() {
            if i > 0 {
                print!(" ");
            }
            print!("{}", v);
        }
        println!("");
    }
}
