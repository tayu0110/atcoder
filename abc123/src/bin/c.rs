#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, a: [usize; 5]}

    let min = *a.iter().min().unwrap();
    if min >= n {
        println!("5");
    } else {
        println!("{}", 5 + (n-1) / min);
    }
}
