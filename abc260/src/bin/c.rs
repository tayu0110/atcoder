#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};



fn main() {
	input! {n: usize, x: usize, y: usize};

    let mut red = vec![0; n];
    let mut blue = vec![0; n];
    red[n-1] = 1;
    for i in (1..n).rev() {
        red[i-1] += red[i];
        blue[i] += red[i] * x;
        red[i-1] += blue[i];
        blue[i-1] += blue[i] * y;
    }

    println!("{}", blue[0]);
}
