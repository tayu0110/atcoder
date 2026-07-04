use proconio::*;
use rustc_hash::FxHashMap;

fn gcd(mut x: i64, mut y: i64) -> i64 {
    while x != 0 {
        y %= x;
        (x, y) = (y, x);
    }
    y
}

fn main() {
    input! {n: usize, p: [(i64, i64); n]}

    let mut memo = FxHashMap::default();
    let mut memo_len = FxHashMap::default();
    for (i, &(x, y)) in p.iter().enumerate() {
        for (_, &(nx, ny)) in p.iter().enumerate().skip(i + 1) {
            let (dy, dx, len) = if nx == x {
                let d = ny.abs_diff(y);
                (1, 0, d * d)
            } else {
                let mut dy = ny - y;
                let mut dx = nx - x;
                let d = dy * dy + dx * dx;
                let g = gcd(dy.abs(), dx.abs());
                dy /= g;
                dx /= g;

                if dx < 0 {
                    dy = -dy;
                    dx = -dx;
                }
                (dy, dx, d as u64)
            };
            // eprintln!("({dy}, {dx}, {len}): ({}, {})", i + 1, j + 1);
            let entry = memo.entry((dy, dx)).or_insert(0usize);
            *entry += 1;

            let entry = memo_len.entry((dy, dx, len)).or_insert(0usize);
            *entry += 1;
        }
    }

    let mut res = 0usize;
    for (_, v) in memo {
        res += v * (v - 1) / 2;
    }

    let mut sub = 0;
    for (_, v) in memo_len {
        sub += v * (v - 1) / 2;
    }
    res -= sub / 2;

    println!("{}", res)
}
