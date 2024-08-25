use proconio::*;

fn main() {
    input! {n: usize, c: i64, a: [i64; n]}

    let mut cum = vec![0; n + 1];
    for i in 0..n {
        cum[i + 1] = cum[i] + a[i];
    }

    let mut res = cum[n];
    if c == 0 {
        res = res.max(0);
    }
    let mut p = 0;
    for i in 1..n + 1 {
        let sum = cum[i] - p;
        res = res.max(cum[n] - sum + sum * c);

        if c > 0 {
            p = p.min(cum[i]);
        } else {
            p = p.max(cum[i]);
        }
    }

    println!("{res}")
}
