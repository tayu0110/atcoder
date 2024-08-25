use proconio::*;

fn main() {
    input! {_: usize, s: marker::Bytes}

    for (i, v) in s.windows(3).enumerate() {
        if v == b"ABC" {
            println!("{}", i + 1);
            return;
        }
    }

    println!("-1")
}
