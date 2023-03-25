use proconio::*;

fn main() {
    input! {n: usize, mut a: [i64; n]}

    let (min, max) = (*a.iter().min().unwrap(), *a.iter().max().unwrap());

    println!("{}", 2 * n - 1);
    if min.abs() <= max.abs() {
        let p = a.iter().position(|&v| v == max).unwrap();
        for i in 0..n {
            println!("{} {}", p + 1, i + 1);
            a[i] += a[p];
        }
        for i in 0..n - 1 {
            println!("{} {}", i + 1, i + 2);
            a[i + 1] += a[i];
        }
    } else {
        let p = a.iter().position(|&v| v == min).unwrap();
        for i in 0..n {
            println!("{} {}", p + 1, i + 1);
            a[i] += a[p];
        }
        for i in (1..n).rev() {
            println!("{} {}", i + 1, i);
            a[i - 1] += a[i];
        }
    }

    assert!(a.windows(2).all(|v| v[0] <= v[1]))
}
