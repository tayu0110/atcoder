use proconio::*;

fn main() {
    input! {n: usize, b: usize, mut c: usize, a: [usize; n]}

    let mut res = 0;
    for a in a.into_iter().rev() {
        res += a * b.min(c);
        c -= b.min(c);
    }

    println!("{res}")
}
