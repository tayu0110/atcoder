use proconio::*;

fn main() {
    input! {n: usize, a: usize, b: usize, h: [usize; n]}

    let diff = a - b;
    let (mut l, mut r) = (0, 1_000_000_010);
    while r - l > 1 {
        let m = (r + l) / 2;
        let mut t = vec![];
        for &h in &h {
            let new = h.saturating_sub(m * b);
            if new > 0 {
                t.push(new);
            }
        }

        let mut k = 0;
        for t in t {
            k += (t + diff - 1) / diff;
        }

        if k <= m {
            r = m;
        } else {
            l = m;
        }
    }

    println!("{}", r);
}
