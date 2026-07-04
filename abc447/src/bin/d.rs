use proconio::*;

fn main() {
    input! {s: marker::Bytes}

    let mut ret = 0;
    let mut a = 0;
    let mut ab = 0;
    for s in s {
        if s == b'A' {
            a += 1;
        } else if s == b'B' {
            if a > 0 {
                ab += 1;
                a -= 1;
            }
        } else {
            if ab > 0 {
                ret += 1;
                ab -= 1;
            }
        }
    }

    println!("{ret}")
}
