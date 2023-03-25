#[allow(unused_imports)]
use proconio::{input, marker::{Chars, Bytes}, source::line::LineSource};
#[allow(unused_imports)]
use itertools::Itertools;

fn main() {
    input! {mut n: usize, mut k: usize, mut a: [usize; n]}
    n += 1;
    a.insert(0, 0);
    a.sort();

    let mut to = vec![0; n];
    for (i, v) in a.iter().enumerate().skip(1) {
        let (mut l, mut r) = (-1, i as i32);

        while r - l > 1 {
            let m = (r + l) / 2;

            if a[m as usize] <= *v / 2 {
                l = m;
            } else {
                r = m;
            }
        }

        to[i] = l as usize;
    }

    let mut dist = vec![0; n];
    for i in 1..n {
        let mut now = i;
        let mut d = 0 as usize;
        while to[now] != now {
            d += 1;
            now = to[now];
        }

        dist[i] = d;
    }

    let mut map = std::collections::HashMap::new();
    for d in dist {
        if d != 0 {
            *map.entry(d).or_insert(0) += 1;
        }
    }

    let mut buf = vec![];
    for (_, v) in map {
        buf.push(v);
    }
    buf.sort();

    let mut res = 0;
    for (i, v) in buf.iter().cloned().enumerate() {
        if v > k {
            println!("{} {}", buf.len() - i, res);
            std::process::exit(0);
        }

        res += v;
        k -= v;
    }

    println!("{} {}", 0, res);
}
