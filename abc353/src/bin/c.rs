use proconio::*;

fn main() {
    input! {n: usize, mut a: [usize; n]}

    a.sort_unstable();

    let mut res = 0;
    let mut sum = a.iter().sum::<usize>();
    const M: usize = 100000000;
    for i in 0..n {
        sum -= a[i];

        let pos = a[i + 1..].partition_point(|k| a[i] + k < M);
        res += sum + a[i] * (n - 1 - i) - M * ((n - 1 - i) - pos);
    }

    println!("{}", res);
}
