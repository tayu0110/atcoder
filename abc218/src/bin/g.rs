fn dfs(now: usize, par: usize, mut diff: i32, target: i64, mut small: i64, mut large: i64, a: &[i64], t: &Vec<Vec<usize>>) -> bool {
    if a[now] < target {
        diff -= 1;
        small = std::cmp::max(small, a[now]);
    } else {
        diff += 1;
        large = std::cmp::min(large, a[now]);
    }

    if t[now].len() == 1 && t[now][0] == par {
        return if diff < 0 {
            !(diff % 2 == 0)
        } else if diff > 0 {
            diff % 2 == 0
        } else {
            (small + large) / 2 >= target
        };
    }

    for &to in &t[now] {
        if to == par {
            continue;
        }
        if dfs(to, now, diff, target, small, large, a, t) {
            return false;
        }
    }

    true
}

fn main() {
    proconio::input! {n: usize, a: [i64; n], p: [(usize, usize); n-1]}

    let mut t = vec![vec![]; n];
    for (u, v) in p {
        t[u-1].push(v-1);
        t[v-1].push(u-1);
    }

    let (mut l, mut r) = (-1, 1000_000_000_000);
    while r - l > 1 {
        let m = (r + l) / 2;

        if dfs(0, 0, 0, m, 0, std::i64::MAX, &a, &t) {
            r = m;
        } else {
            l = m;
        }
    }

    println!("{}", l);
}