use std::collections::VecDeque;

use proconio::*;

fn main() {
    input! {mut h: usize, w: usize, q: usize, a: [marker::Chars; h], mut query: [(usize, usize, char); q]}

    let mut a = if h < w {
        a.into_iter()
            .map(|s| s.into_iter().collect::<VecDeque<char>>())
            .collect::<Vec<_>>()
    } else {
        let mut buf = vec![];
        for i in 0..w {
            let mut b = VecDeque::new();
            for j in 0..h {
                b.push_back(a[j][i]);
            }
            buf.push(b);
        }
        h = w;
        for q in query.iter_mut() {
            q.0 = 3 - q.0;
        }
        buf
    };

    let mut res = String::new();
    for (ty, p, c) in query {
        if ty == 1 {
            res.push(a[p - 1].pop_back().unwrap());
            a[p - 1].push_front(c);
        } else {
            res.push(a[h - 1][p - 1]);
            for i in (1..h).rev() {
                a[i][p - 1] = a[i - 1][p - 1];
            }
            a[0][p - 1] = c;
        }
    }

    println!("{res}")
}
