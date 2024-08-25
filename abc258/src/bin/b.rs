#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, a: [Chars; n]};
    let a = {
        let mut buf = vec![];
        for v in a {
            let mut t = vec![];
            for w in v {
                t.push((w as u8 - b'0') as usize);
            }
            buf.push(t);
        }
        buf
    };

    let mut a = a;
    for i in 0..n {
        let mut b = a[i].clone();
        a[i].append(&mut b);
    }

    let a = {
        let mut b = a.clone();
        a.append(&mut b);
        a
    };

    let x = [1, 0, -1, 0, 1, 1, -1, -1];
    let y = [0, 1, 0, -1, 1, -1, 1, -1];

    let mut res = 0;
    for i in 0..n*2 {
        for j in 0..n*2 {
            for k in 0..8 {
                let mut nx = i as i32;
                let mut ny = j as i32;
                let mut tmp = 0usize;
                let mut bad = false;
                for _ in 0..n {
                    if nx < 0 || ny < 0 || nx as usize >= 2*n || ny as usize >= 2*n {
                        bad = true;
                        break;
                    }
                    tmp = tmp * 10 + a[ny as usize][nx as usize];
                    ny += y[k];
                    nx += x[k];
                }
                if bad {
                    continue;
                }
                res = std::cmp::max(res, tmp);
            }
        }
    }

    println!("{}", res);
}
