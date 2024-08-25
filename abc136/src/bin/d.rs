#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {s: Chars};
    let n = s.len();

    let mut v = vec![vec![1, 0]; n];
    for i in 0..n {
        if s[i] == 'R' {
            if s[i + 1] == 'R' {
                v[i + 1][1] += v[i][0];
                v[i][0] = 0;
            }
            v[i + 1][0] += v[i][1];
            v[i][1] = 0;
        }
    }
    // eprintln!("v: {:?}", v);
    for i in (0..n).rev() {
        if s[i] == 'L' {
            if s[i - 1] == 'L' {
                v[i - 1][1] += v[i][0];
                v[i][0] = 0;
            }
            v[i - 1][0] += v[i][1];
            v[i][1] = 0;
        }
    }
    // eprintln!("v: {:?}", v);

    for (i, v) in v.iter().enumerate().take(n) {
        if i > 0 {
            print!(" ");
        }
        print!("{}", v[0]);
    }
    println!();
}
