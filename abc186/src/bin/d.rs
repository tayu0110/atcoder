use proconio::*;

fn main() {
    input! {n: usize, mut a: [i64; n]}

    a.sort();
    let mut res = 0;
    let mut sum = 0;
    for (i, &a) in a.iter().enumerate() {
        res += (a * i as i64) - sum;
        sum += a;
    }

    println!("{}", res)
}
