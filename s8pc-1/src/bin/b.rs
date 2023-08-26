use proconio::*;

fn main() {
    input! {h: usize, w: usize, n: usize, p: [(usize, usize); n]}

    let mut res = vec![];
    for i in 1..w {
        let (mut f, mut s) = (0, 0);
        for &(x, y) in &p {
            if x * h > y * i {
                f += 1;
            } else if x * h < y * i {
                s += 1;
            }
        }

        if f == s && f + s == n {
            res.push((i, h))
        }
    }
    for j in 1..=h {
        let (mut f, mut s) = (0, 0);
        for &(x, y) in &p {
            if x * j > y * w {
                f += 1;
            } else if x * j < y * w {
                s += 1;
            }
        }

        if f == s && f + s == n {
            res.push((w, j))
        }
    }

    if res.is_empty() {
        println!("-1")
    } else {
        for (x, y) in res {
            println!("({},{})", x, y)
        }
    }
}
