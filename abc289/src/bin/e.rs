#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {t: usize}

    let mut res = vec![];
    for _ in 0..t {
        input! {n: usize, m: usize, c: [u8; n], p:[(usize, usize); m]}

        let mut t = vec![vec![]; n];
        for (u, v) in p {
            t[u - 1].push(v - 1);
            t[v - 1].push(u - 1);
        }

        let mut nt = std::collections::VecDeque::new();
        nt.push_back((0, n - 1, 0));

        let mut reached = vec![vec![std::usize::MAX; n]; n];
        while let Some((tak, aok, nd)) = nt.pop_front() {
            if reached[tak][aok] != std::usize::MAX {
                continue;
            }
            reached[tak][aok] = nd;

            for &to in &t[tak] {
                for &to2 in &t[aok] {
                    if c[to] == c[to2] {
                        continue;
                    }

                    if reached[to][to2] != std::usize::MAX {
                        continue;
                    }

                    nt.push_back((to, to2, nd + 1));
                }
            }
        }

        if reached[n - 1][0] == std::usize::MAX {
            res.push(-1);
        } else {
            res.push(reached[n - 1][0] as i32);
        }
    }

    for res in res {
        println!("{}", res);
    }
}
