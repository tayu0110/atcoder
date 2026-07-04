use num::Integer;
use proconio::*;

fn main() {
    input! {n: usize, x: i64, y: i64}

    let (mut min, mut max) = (0, i64::MAX);
    for _ in 0..n {
        input! {a: i64, b: i64}
        // ys + x <= bs + a
        // (y-b)s <= a-x
        let p = y - b;
        let q = a - x;
        if p == 0 {
            if a < x {
                println!("No");
                return;
            }
        } else if p > 0 {
            max = max.min(q.div_floor(&p));
        } else {
            min = min.max(q.div_ceil(&p));
        }
    }
    if min <= max {
        println!("Yes")
    } else {
        println!("No")
    }
}
