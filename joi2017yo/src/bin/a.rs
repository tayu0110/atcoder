use proconio::*;

fn main() {
    input! {a: i32, b: i32, c: i32, d: i32, e: i32}

    let mut res = 0;
    for now in a..b {
        if now < 0 {
            res += c;
        } else if now == 0 {
            res += d;
            res += e;
        } else {
            res += e;
        }
    }

    println!("{res}")
}
