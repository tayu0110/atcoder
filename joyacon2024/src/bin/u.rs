use proconio::*;

fn main() {
    input! {n: usize, d: [usize; n]}

    let mut sum = 0;
    let mut res = 0;
    for d in d {
        res += sum * d;
        sum += d;
    }

    println!("{res}")
}
