use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, x: i64, mut a: [i64; n]}

    let mut nx = a.iter().filter(|&&a| a >= 0).fold(0, |s, v| s ^ v) ^ x;
    let m = a.iter().filter(|&&a| a < 0).count();

    let mut res = vec![];
    for _ in 0..m {
        let mut sum = 0;
        for i in (0..40).rev() {
            if nx & (1 << i) == 0 {
                continue;
            }

            if 1 << i > x {
                println!("-1");
                return;
            }

            if sum + (1 << i) <= x {
                sum += 1 << i;
            }
        }

        nx ^= sum;
        res.push(sum);
    }

    if nx != 0 {
        println!("-1");
        return;
    }

    a.iter_mut()
        .filter(|a| **a < 0)
        .for_each(|a| *a = res.pop().unwrap());
    println!("{}", a.iter().join(" "))
}
