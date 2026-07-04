use proconio::*;
fn main() {
    input! {n: usize, k: usize}
    let mut v = vec![0i64; k];
    let mut r = vec![n; k];
    let max = 100_000_000_000_000i64;
    v[0] = max;
    r[0] = 1;
    for i in 0..n - 1 {
        v[2 * i + 1] = max / 2 + i as i64 + 1;
        v[2 * i + 2] = max / 2 - i as i64 - 1;
        r[2 * i + 1] = i + 2;
        r[2 * i + 2] = i + 2;
    }
    let mut sum = 0;
    for i in 2 * n - 1..k {
        v[i] = (i + 2 - 2 * n) as i64;
        sum += v[i];
    }
    v[2 * n - 2] -= sum;
    for i in 0..k {
        print!("{}{}", v[i], if i + 1 == k { "\n" } else { " " });
    }
    println!("YES");
    for i in 0..k {
        print!("{}{}", r[i], if i + 1 == k { "\n" } else { " " });
    }
}
