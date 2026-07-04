use proconio::*;
use rand::{thread_rng, Rng};

const MAX_THRESH: i64 = 1000_000_000_000_000_000;
const MIN_THRESH: i64 = -1000_000_000_000_000_000;

fn gcd(mut x: i64, mut y: i64) -> i64 {
    while y != 0 {
        x %= y;
        (x, y) = (y, x);
    }
    x
}

fn main() {
    input! {n: usize, p: [(i64, i64); n]}

    let mut rng = thread_rng();
    for _ in 0..50 {
        let i = rng.gen_range(0..n);
        let j = rng.gen_range(0..n);
        if i == j {
            continue;
        }

        let (x, y) = p[i];
        let (nx, ny) = p[j];

        let (a, b, c) = if x == nx {
            (1, 0, -x)
        } else if y == ny {
            (0, 1, -y)
        } else {
            let mut dx = nx - x;
            let mut dy = ny - y;
            let g = gcd(dx, dy);
            dx /= g;
            dy /= g;
            if dy < 0 {
                dy = -dy;
                dx = -dx;
            }

            let a = dy;
            let b = -dx;
            let c = dx * y - dy * x;
            if !(MIN_THRESH..=MAX_THRESH).contains(&a)
                || !(MIN_THRESH..=MAX_THRESH).contains(&b)
                || !(MIN_THRESH..=MAX_THRESH).contains(&c)
            {
                continue;
            }
            (a, b, c)
        };

        let mut cnt = 0;
        for &(x, y) in &p {
            if a as i128 * x as i128 + b as i128 * y as i128 + c as i128 == 0 {
                cnt += 1;
            }
        }

        if cnt * 2 > n {
            println!("Yes");
            println!("{a} {b} {c}");
            return;
        }
    }

    println!("No");
}
