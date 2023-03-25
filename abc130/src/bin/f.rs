#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, p: [(i64, i64, char); n]};

    const INF: i64 = 0x3f3f3f3f3f3f3f3f;
    let mut xmax = std::collections::HashMap::new();
    let (mut xmin, mut ymax, mut ymin) = (xmax.clone(), xmax.clone(), xmax.clone());

    for c in "RLUD".chars() {
        xmax.insert(c, -INF);
        xmin.insert(c, INF);
        ymax.insert(c, -INF);
        ymin.insert(c, INF);
    }

    for (x, y, d) in p {
        *xmax.entry(d).or_insert(-INF) = std::cmp::max(*xmax.get(&d).unwrap(), x);
        *xmin.entry(d).or_insert(INF) = std::cmp::min(*xmin.get(&d).unwrap(), x);
        *ymax.entry(d).or_insert(-INF) = std::cmp::max(*ymax.get(&d).unwrap(), y);
        *ymin.entry(d).or_insert(INF) = std::cmp::min(*ymin.get(&d).unwrap(), y);
    }

    let xmax = "RLUD".chars().fold(vec![], |mut v, c| { v.push(*xmax.get(&c).unwrap() as f64); v });
    let xmin = "RLUD".chars().fold(vec![], |mut v, c| { v.push(*xmin.get(&c).unwrap() as f64); v });
    let ymax = "RLUD".chars().fold(vec![], |mut v, c| { v.push(*ymax.get(&c).unwrap() as f64); v });
    let ymin = "RLUD".chars().fold(vec![], |mut v, c| { v.push(*ymin.get(&c).unwrap() as f64); v });


    let dir = [1.0, -1.0, 1.0, -1.0];
    let (mut l, mut r) = (0.0, 5e8);
    let mut cnt = 0;
    while r - l > 1e-10 {
        let m = vec![(2.0*r + l) / 3.0, (r + 2.0*l) / 3.0];
        let mut res = vec![0.0, 0.0];
        for j in 0..2 {
            let (mut xx, mut xn, mut yx, mut yn) = (-INF as f64, INF as f64, -INF as f64, INF as f64);
            for i in 0..2 {
                if xx < xmax[i] + dir[i] * m[j] {
                    xx = xmax[i] + dir[i] * m[j];
                }
                if xn > xmin[i] + dir[i] * m[j] {
                    xn = xmin[i] + dir[i] * m[j];
                }
                if yx < ymax[i] {
                    yx = ymax[i];
                }
                if yn > ymin[i] {
                    yn = ymin[i];
                }
            }
            for i in 2..4 {
                if xx < xmax[i] {
                    xx = xmax[i];
                }
                if xn > xmin[i] {
                    xn = xmin[i];
                }
                if yx < ymax[i] + dir[i] * m[j] {
                    yx = ymax[i] + dir[i] * m[j];
                }
                if yn > ymin[i] + dir[i] * m[j] {
                    yn = ymin[i] + dir[i] * m[j];
                }
            }    
            res[j] = (xx - xn) * (yx - yn);
        }

        if res[0] < res[1] {
            l = m[1];
        } else {
            r = m[0];
        }

        cnt += 1;
        if cnt > 50000 {
            break;
        }
    }

    let (mut xx, mut xn, mut yx, mut yn) = (-INF as f64, INF as f64, -INF as f64, INF as f64);
    for i in 0..2 {
        if xx < xmax[i] + dir[i] * r {
            xx = xmax[i] + dir[i] * r;
        }
        if xn > xmin[i] + dir[i] * r {
            xn = xmin[i] + dir[i] * r;
        }
        if yx < ymax[i] {
            yx = ymax[i];
        }
        if yn > ymin[i] {
            yn = ymin[i];
        }
    }
    for i in 2..4 {
        if xx < xmax[i] {
            xx = xmax[i];
        }
        if xn > xmin[i] {
            xn = xmin[i];
        }
        if yx < ymax[i] + dir[i] * r {
            yx = ymax[i] + dir[i] * r;
        }
        if yn > ymin[i] + dir[i] * r {
            yn = ymin[i] + dir[i] * r;
        }
    }    
    let res = (xx - xn) * (yx - yn);

    
    println!("{}", res);
}
