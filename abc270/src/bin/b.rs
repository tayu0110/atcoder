#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {mut x: i32, mut y: i32, mut z: i32}

    if x < 0 {
        x = -x;
        y = -y;
        z = -z;
    }

    if y < 0 || y > x {
        println!("{}", x);
    } else if z < y {
        println!("{}", z.abs() + (x - z).abs());
    } else {
        println!("-1");
    }
}
