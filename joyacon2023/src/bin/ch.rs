use itertools::Itertools;
use proconio::input;

fn main() {
    input! {h: usize, w: usize, n: usize, r: usize, c: usize, a: [[usize; w]; h]}

    let mut up = vec![std::usize::MAX; n + 1];
    let mut down = vec![0; n + 1];
    let mut left = vec![std::usize::MAX; n + 1];
    let mut right = vec![0; n + 1];
    let mut set = std::collections::HashSet::new();

    for i in 0..h {
        for j in 0..w {
            let now = a[i][j];
            set.insert(now);
            up[now] = std::cmp::min(i, up[now]);
            down[now] = std::cmp::max(i, down[now]);
            left[now] = std::cmp::min(j, left[now]);
            right[now] = std::cmp::max(j, right[now]);
        }
    }

    for i in 0..=h - r {
        let mut res = vec![];
        for j in 0..=w - c {
            let (u, d, l, r) = (i, i + r - 1, j, j + c - 1);

            let mut v = set.len();
            for k in 0..=n {
                if !set.contains(&k) {
                    continue;
                }

                if u <= up[k] && down[k] <= d && l <= left[k] && right[k] <= r {
                    v -= 1;
                }
            }

            res.push(v);
        }

        println!("{}", res.iter().join(" "))
    }
}
