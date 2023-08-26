use proconio::*;

fn main() {
    input! {mut h: usize, a: usize, b: usize, c: usize, d: usize}

    let mut res = (h + a - 1) / a * b;
    let mut now = 0;
    while h > 0 {
        now += d;
        h = h.saturating_sub(c);
        if h > 0 {
            h -= h / 2;
        }
        res = res.min(now + (h + a - 1) / a * b);
    }

    println!("{}", res)
}
