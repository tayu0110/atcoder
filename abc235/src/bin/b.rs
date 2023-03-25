#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, h: [usize; n]}

    for v in h.windows(2) {
        if v[0] >= v[1] {
            println!("{}", v[0]);
            std::process::exit(0);
        }
    }
    println!("{}", h.last().unwrap());
}
