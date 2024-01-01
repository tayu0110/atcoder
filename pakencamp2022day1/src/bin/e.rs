use proconio::*;

fn main() {
    input! {h: f64, w: f64}

    let (mut l, mut r) = (0.0, 10000000000.0);
    for _ in 0..1000 {
        let m = (r + l) / 2.0;
        if m * 2.0 > h || m * 2.0 > w {
            r = m;
            continue;
        }

        let d = (w - m * 2.0).hypot(h - m * 2.0);
        if d >= m * 2.0 {
            l = m;
        } else {
            r = m;
        }
    }

    println!("{l:.20}");
}
