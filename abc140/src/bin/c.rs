use proconio::*;

fn main() {
    input! {n: usize, b: [usize; n - 1]}

    let mut a = vec![usize::MAX; n];
    for i in 0..n - 1 {
        a[i] = a[i].min(b[i]);
        a[i + 1] = b[i];
    }

    println!("{}", a.iter().sum::<usize>())
}
