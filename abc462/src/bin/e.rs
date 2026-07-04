use proconio::*;

fn main() {
    input! {t: usize}

    for _ in 0..t {
        input! {mut a: usize, mut b: usize, x: i64, y: i64}
        let mut x = x.abs() as usize;
        let mut y = y.abs() as usize;
        if x < y {
            (x, y) = (y, x);
            (a, b) = (b, a);
        }

        let mut ret = y * 2 * a.min(b);
        x -= y;
        if x == 0 {
            println!("{}", ret);
            continue;
        }

        let (mut l, mut r) = (0, 2000_000_000);
        while l + 1 < r {
            let m1 = (l * 2 + r) / 3;
            let m2 = (l + r * 2) / 3;

            if m2 * 2 + 1 > x {
                r = m2;
                continue;
            }
        }
    }
}
