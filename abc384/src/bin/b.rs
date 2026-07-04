use proconio::*;

fn main() {
    input! {n: usize, mut r: i32}

    for _ in 0..n {
        input! {d: usize, a: i32}

        if d == 1 {
            if 1600 <= r && r < 2800 {
                r += a;
            }
        } else {
            if 1200 <= r && r < 2400 {
                r += a;
            }
        }
    }

    println!("{r}")
}
