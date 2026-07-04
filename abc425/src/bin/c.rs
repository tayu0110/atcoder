use proconio::*;

fn main() {
    input! {n: usize, q: usize, a: [usize; n]}

    let mut base = 0;
    let mut cum = vec![0; 2 * n + 1];
    for i in 0..n * 2 {
        cum[i + 1] = cum[i] + a[i % n];
    }
    for _ in 0..q {
        input! {ty: u8}

        if ty == 1 {
            input! {c: usize}
            base += c;
            base %= n;
        } else {
            input! {mut l: usize, mut r: usize}
            r += base;
            l += base;
            l -= 1;
            println!("{}", cum[r] - cum[l])
        }
    }
}
