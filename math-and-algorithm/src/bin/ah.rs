use proconio::*;

fn main() {
    input! {mut x1: i64, mut y1: i64, mut x2: i64, mut y2: i64, mut x3: i64, mut y3: i64, mut x4: i64, mut y4: i64}

    let mut res = true;
    for _ in 0..2 {
        let s = (x2 - x1) * (y3 - y1) - (x3 - x1) * (y2 - y1);
        let t = (x2 - x1) * (y4 - y1) - (x4 - x1) * (y2 - y1);
        res &= s.signum() * t.signum() <= 0;
        (x1, y1, x2, y2, x3, y3, x4, y4) = (x3, y3, x4, y4, x1, y1, x2, y2);
    }
    if res {
        println!("Yes")
    } else {
        println!("No")
    }
}
