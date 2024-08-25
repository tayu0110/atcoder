use proconio::*;

fn main() {
    input! {n: usize, k: usize, mut a: [usize; n]}

    a.sort_unstable();
    println!(
        "{}",
        a.windows(n - k)
            .map(|v| v.last().unwrap() - v.first().unwrap())
            .min()
            .unwrap()
    );
}
