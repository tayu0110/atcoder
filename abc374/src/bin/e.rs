use proconio::*;

fn main() {
    input! {n: usize, x: usize, device: [(usize, usize, usize, usize); n]}

    let (mut l, mut r) = (0, 1000_000_000_000);
    while r - l > 1 {
        let w = (r + l) / 2;
        let mut sum = 0;
        for &(mut a, mut p, mut b, mut q) in &device {
            if b * p < a * q {
                (a, p, b, q) = (b, q, a, p);
            }
            let mut min = w / b * q;
            let rem = w - w / b * b;
            if (rem + b - 1) / b * q > (rem + a - 1) / a * p {
                eprintln!("a: {}, b: {}", (rem + a - 1) / a, w / b);
                min += (rem + a - 1) / a * p;
            } else {
                eprintln!("a: 0, b: {}", (w + b - 1) / b);
                min += (rem + b - 1) / b * q;
            }
            sum += min;
        }
        eprintln!("w: {w}, sum: {sum}");

        if sum <= x {
            l = w;
        } else {
            r = w;
        }
    }

    println!("{}", l);
}
