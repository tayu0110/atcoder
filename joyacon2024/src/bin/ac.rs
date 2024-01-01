use proconio::*;

fn main() {
    input! {x: marker::Bytes}

    println!("{}", x[0] + x[1] - b'0' * 2);
}
