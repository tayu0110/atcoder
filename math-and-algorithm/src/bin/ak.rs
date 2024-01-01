use proconio::*;

fn main() {
    input! {n: usize, mut a: [usize; n - 1], m: usize, b: [usize; m]}
    a.insert(0, 0);

    for i in 0..n - 1 {
        a[i + 1] += a[i];
    }

    println!(
        "{}",
        b.windows(2)
            .map(|v| a[v[0].max(v[1]) - 1] - a[v[0].min(v[1]) - 1])
            .sum::<usize>()
    )
}
