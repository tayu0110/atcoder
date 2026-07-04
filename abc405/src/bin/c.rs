use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut sum = 0;
    let mut res = 0usize;
    for a in a {
        res += a * sum;
        sum += a;
    }

    println!("{res}")
}
