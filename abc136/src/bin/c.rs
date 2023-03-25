#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, mut h: [usize; n]};

    h.insert(0, 0);
    for i in 1..=n {
        if h[i] < h[i-1] {
            println!("No");
            std::process::exit(0);
        }

        if h[i] > h[i-1] {
            h[i] -= 1;
        }
    }

    println!("Yes");
}
