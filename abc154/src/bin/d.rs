use proconio::*;

fn main() {
    input! {n: usize, k: usize, p: [usize; n]}
    let mut res = p.iter().take(k).map(|&p| p + 1).sum::<usize>() as f64 / 2.0;
    let mut sum = res;
    for i in k..n {
        sum += (p[i] + 1) as f64 / 2.0;
        sum -= (p[i - k] + 1) as f64 / 2.0;
        if res < sum {
            res = sum;
        }
    }

    println!("{}", res)
}
