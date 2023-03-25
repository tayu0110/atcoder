#[allow(unused_imports)]
use proconio::{input, marker::{Chars, Bytes}, source::line::LineSource};
#[allow(unused_imports)]
use itertools::Itertools;

fn main() {
    input! {n: i64, m: i64}

    let mut map = std::collections::HashMap::new();
    for i in 0..=10000i64 {
        map.insert(i*i, i);
    }

    let mut dist = vec![vec![-1i64; n as usize]; n as usize];
    let mut nt = std::collections::VecDeque::new();
    nt.push_back((0, 1, 1));

    while let Some((nd, r, c)) = nt.pop_front() {
        if dist[r-1][c-1] != -1 {
            continue;
        }
        dist[r-1][c-1] = nd;

        for i in 1..=n {
            let m = m - (r as i64 - i) * (r as i64 - i);
            if m < 0 {
                continue;
            }

            if let Some(l) = map.get(&m) {
                let l = *l;
                if 1 <= l+c as i64 && l+c as i64 <= n && dist[i as usize -1][(l+c as i64) as usize -1] == -1 {
                    nt.push_back((nd+1, i as usize, (l+c as i64) as usize));
                }
                if 1 <= c as i64-l && c as i64 - l <= n && dist[i as usize -1][(c as i64 - l) as usize -1] == -1 {
                    nt.push_back((nd+1, i as usize, (c as i64 - l) as usize));
                }
            }

        }
    }

    for v in dist {
        println!("{}", v.iter().join(" "));
    }
}
