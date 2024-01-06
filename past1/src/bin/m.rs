use proconio::*;

fn solve(p: &[(f64, f64)], q: Option<(f64, f64)>) -> f64 {
    let mut p = p.to_vec();
    if let Some(q) = q {
        p.push(q);
    }

    let (mut l, mut r) = (0.0, 5000000.0);
    while r - l > 1e-8 {
        let m = (r + l) / 2.0;
        p.sort_unstable_by(|a, b| (b.1 - m * b.0).partial_cmp(&(a.1 - m * a.0)).unwrap());

        let s = p[..5]
            .iter()
            .fold((0.0, 0.0), |(s, t), (a, b)| (s + b, t + a));
        let s = s.0 / s.1;
        if s >= m {
            l = m;
        } else {
            r = m;
        }
    }

    p.sort_unstable_by(|a, b| (b.1 - l * b.0).partial_cmp(&(a.1 - l * a.0)).unwrap());
    let s = p[..5]
        .iter()
        .fold((0.0, 0.0), |(s, t), (a, b)| (s + b, t + a));
    s.0 / s.1
}

fn main() {
    input! {n: usize, m: usize, p: [(f64, f64); n], q: [(f64, f64); m]}

    let mut res = solve(&p, None);
    for q in q {
        res = res.max(solve(&p, Some(q)));
    }

    println!("{res}");
}
