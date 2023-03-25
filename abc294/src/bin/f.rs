use proconio::input;

fn main() {
    input! {n: usize, m: usize, k: usize, p: [(i64, i64); n], q: [(i64, i64); m]}

    const DEN: f64 = 1.0;
    let (mut l, mut r) = (0.0, DEN + 1e-12);
    while r - l > 1e-12 {
        let mid = (r + l) / 2.0;

        let mut a = p
            .iter()
            .map(|&(a, b)| (1.0 - mid) * a as f64 - mid * b as f64)
            .collect::<Vec<_>>();
        let mut b = q
            .iter()
            .map(|&(c, d)| (mid - 1.0) * c as f64 + mid * d as f64)
            .collect::<Vec<_>>();

        a.sort_by(|l, r| l.partial_cmp(r).unwrap());
        b.sort_by(|l, r| l.partial_cmp(r).unwrap());

        let mut nk = 0;
        for a in &a {
            let (mut l, mut r) = (-1, m as i32);
            while r - l > 1 {
                let z = (r + l) / 2;
                if a - b[z as usize] >= 0.0 {
                    l = z;
                } else {
                    r = z;
                }
            }

            nk += r as usize;
        }

        if nk < k {
            r = mid;
        } else {
            l = mid;
        }
    }

    println!("{}", r / DEN * 100.0)
}
