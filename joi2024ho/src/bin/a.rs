use proconio::*;

fn main() {
    input! {n: usize, t: usize, mut a: [usize; n]}

    a.sort_unstable();

    let k = a[n / 2];
    println!("{}", a.into_iter().map(|a| a.abs_diff(k)).sum::<usize>())
}
