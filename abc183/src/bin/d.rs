#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, w: i64, p: [(usize, usize, i64); n]}

    let mut v = vec![0; 200010];
    for (s, t, p) in p {
        v[s] += p;
        v[t] -= p;
    }

    for i in 0..200005 {
        if w < v[i] {
            println!("No");
            std::process::exit(0);
        }
        v[i+1] += v[i];
    }

    println!("Yes");
}
