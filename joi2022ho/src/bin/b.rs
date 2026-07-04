use proconio::*;

fn main() {
    input! {n: usize, m: usize, mut a: [usize; n], b: [usize; n]}
    a.iter_mut()
        .zip(b.iter())
        .for_each(|(a, b)| *a = (*a).max(*b));

    let (mut l, mut r) = (0, 4000_000_000_000_000_000);
    while r - l > 1 {
        let min = (r + l) / 2;
        let mut rem = 0;
        for &a in &a {
            let need = (min + a - 1) / a;
            rem += m.saturating_sub(need);
        }

        let mut s = 0;
        for (&a, &b) in a.iter().zip(b.iter()) {
            let need = (min + a - 1) / a;
            if need > m {
                let t = min - a * m;
                s += (t + b - 1) / b;
            }
        }

        if s <= rem {
            l = min;
        } else {
            r = min;
        }
    }

    println!("{l}")
}
