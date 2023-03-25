#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, mut p: [usize; n-1]};

    let mut v = vec![0];
    v.append(&mut p);

    let mut now = n-1;
    for i in 1..n+1 {
        if v[now] == 1 {
            println!("{}", i);
            std::process::exit(0);
        }

        now = v[now] - 1;
    }
}
