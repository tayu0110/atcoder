use proconio::*;

fn main() {
    input! {t: usize, a: usize, b: usize, c: usize, d: usize}

    if a + c <= t {
        println!("{}", b + d)
    } else if c <= t {
        println!("{}", d)
    } else if a <= t {
        println!("{}", b)
    } else {
        println!("0")
    }
}
