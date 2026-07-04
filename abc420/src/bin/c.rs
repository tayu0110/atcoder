use proconio::*;

fn main() {
    input! {n: usize, q: usize, mut a: [usize; n], mut b: [usize; n]}

    let mut sum = 0;
    for i in 0..n {
        sum += a[i].min(b[i]);
    }

    for _ in 0..q {
        input! {c: char, x: usize, v: usize}
        sum -= a[x - 1].min(b[x - 1]);
        if c == 'A' {
            a[x - 1] = v;
        } else {
            b[x - 1] = v;
        }
        sum += a[x - 1].min(b[x - 1]);
        println!("{sum}")
    }
}
