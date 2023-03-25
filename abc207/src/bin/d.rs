#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};


fn main() {
    input! {n: usize, mut a: [(i32, i32); n], mut c: [(i32, i32); n]}

    for i in 0..n {
        c[i] = (c[i].0 - c[0].0, c[i].1 - c[0].1);
    }

    for i in 0..n {
        let mut b = a.clone();
        for j in 0..n {
            b[j] = (b[j].0 - b[i].0, b[j].1 - b[i].0);
        }

        for j in 0..n {
            let (bx, by) = b[j];
            if bx == 0 && by == 0 {
                continue;
            }
            for k in 0..n {
                let (cx, cy) = c[k];
                if bx * bx + by * by == cx * cx + cy * cy {
                    let mut d = c.clone();
                    for l in 0..n {

                    }
                }
            }
        }
    }
}
