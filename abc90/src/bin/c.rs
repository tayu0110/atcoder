#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {mut n: usize, mut m: usize}

    if n > m {
        std::mem::swap(&mut n, &mut m);
    }

    if n == 1 && m == 1 {
        println!("1");
    } else if n == 1 {
        println!("{}", m - 2);
    } else {
        println!("{}", n * m - n * 2 - (m-2) * 2);
    }

}
