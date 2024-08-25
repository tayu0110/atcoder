use proconio::*;

fn main() {
    input! {a: usize, b: usize, c: usize, d: usize}

    if b * c > a * d {
        println!("TAKAHASHI")
    } else if b * c == a * d {
        println!("DRAW")
    } else {
        println!("AOKI")
    }
}
