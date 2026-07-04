use proconio::*;

fn solve(h: usize, x: usize, p: &[(usize, usize)]) -> bool {
    let (mut lo, mut hi) = (0, usize::MAX);
    for &(u, d) in p {
        if h > u + d {
            return false;
        }

        let diff = u + d - h;
        let min = diff.saturating_sub(d);
        let max = diff.min(u);

        let (mut min, mut max) = (u - max, u - min);
        if max.saturating_add(x) < lo || hi.saturating_add(x) < min {
            return false;
        }

        if min <= lo && hi <= max {
            min = lo.saturating_sub(x).max(min);
            max = hi.saturating_add(x).min(max);
        } else if lo < min && max < hi {
            // no op
        } else if lo <= min && hi <= max {
            max = hi.saturating_add(x).min(max);
        } else {
            min = lo.saturating_sub(x).max(min);
        }
        assert!(min <= max);
        (lo, hi) = (min, max)
    }
    true
}

fn main() {
    input! {n: usize, x: usize, p: [(usize, usize); n]}

    let maximum = p.iter().map(|(u, d)| u + d).min().unwrap();
    let (mut l, mut r) = (0, maximum + 1);
    while r - l > 1 {
        let h = (r + l) / 2;
        if solve(h, x, &p) {
            l = h;
        } else {
            r = h;
        }
    }

    println!("{}", p.into_iter().map(|(u, d)| u + d - l).sum::<usize>())
}
