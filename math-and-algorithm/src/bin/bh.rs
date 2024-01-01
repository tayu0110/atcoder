use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut res = 0;
    let mut sum = 0;
    for (i, a) in a.into_iter().enumerate() {
        res += a * i - sum;
        sum += a;
    }

    println!("{res}")
}
