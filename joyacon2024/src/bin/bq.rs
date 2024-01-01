use proconio::*;

fn main() {
    input! {a: marker::Bytes, b: marker::Bytes, c: marker::Bytes}

    if a.last().unwrap() == &b[0] && b.last().unwrap() == &c[0] {
        println!("YES")
    } else {
        println!("NO")
    }
}
