use proconio::input;

fn main() {
    input! {a: u128, b: u128}

    let (mut l, mut r) = (0, 1000_000_000_000);
    while r - l > 2 {
        let g1 = (l * 2 + r) / 3;
        let g2 = (l + r * 2) / 3;

        let t1 = (a as f64) / (g1 as f64).sqrt() + (b * (g1 - 1)) as f64;
        let t2 = (a as f64) / (g2 as f64).sqrt() + (b * (g2 - 1)) as f64;

        if t1 < t2 {
            r = g2;
        } else {
            l = g1;
        }
    }

    let mut res = std::f64::MAX;
    for g in l.saturating_sub(10) + 1..=r + 10 {
        let r = (a as f64) / (g as f64).sqrt() + (b * (g - 1)) as f64;
        if r < res {
            res = r;
        }
    }

    println!("{}", res);
}
