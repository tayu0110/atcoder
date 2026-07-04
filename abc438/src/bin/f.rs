use proconio::*;

fn dfs(
    now: usize,
    par: usize,
    t: &[Vec<usize>],
    memo: &mut [Vec<usize>],
    cnt: &mut [Vec<usize>],
) -> (usize, usize) {
    let mut ret = now;
    let mut c = 1;
    for i in 0..t[now].len() {
        if t[now][i] == par {
            continue;
        }

        (memo[now][i], cnt[now][i]) = dfs(t[now][i], now, t, memo, cnt);
        ret = ret.min(memo[now][i]);
        c += cnt[now][i];
    }
    (ret, c)
}
fn dfs2(
    now: usize,
    prop: (usize, usize),
    t: &[Vec<usize>],
    memo: &mut [Vec<usize>],
    cnt: &mut [Vec<usize>],
) {
    let mut p = prop.0.min(now);
    let mut par = usize::MAX;
    let mut c = 1;
    for i in 0..t[now].len() {
        if memo[now][i] == usize::MAX {
            par = i;
            memo[now][i] = prop.0;
            cnt[now][i] = prop.1;
        }
        p = p.min(memo[now][i]);
        c += cnt[now][i];
    }
    for i in 0..t[now].len() {
        if par != i {
            dfs2(t[now][i], (p, c - cnt[now][i]), t, memo, cnt);
        }
    }
}

fn main() {
    input! {n: usize, e: [(usize, usize); n - 1]}

    let mut t = vec![vec![]; n];
    let mut memo = vec![vec![]; n];
    let mut cnt = vec![vec![]; n];
    for (u, v) in e {
        t[u].push(v);
        t[v].push(u);
        memo[u].push(usize::MAX);
        memo[v].push(usize::MAX);
        cnt[u].push(0);
        cnt[v].push(0);
    }

    dfs(0, 0, &t, &mut memo, &mut cnt);
    dfs2(0, (usize::MAX, 0), &t, &mut memo, &mut cnt);

    let mut children = vec![0; n];
    for i in 0..n {
        for j in 0..cnt[i].len() {
            children[i] += cnt[i][j];
        }
    }

    let mut ret = 1;
    let (mut start, mut end) = (0, 0);
    let mut check = vec![false; n];
    check[0] = true;
    for i in 0..cnt[0].len() {
        ret += cnt[0][i] * (children[0] - cnt[0][i]);
        ret += cnt[0][i];
    }

    for i in 1..n {
        if check[i] {
            ret += children[start] * children[end];
            continue;
        }

        let mut min_cnt = 0;
        for j in 0..memo[i].len() {
            if memo[i][j] < i {
                min_cnt += 1;
            }
        }

        if min_cnt > 1 {
            break;
        }

        let mut now = i;
        while now >= i {
            check[now] = true;
            for j in 0..memo[now].len() {
                if memo[now][j] < i {
                    now = t[now][j];
                    break;
                }
            }
        }

        if now < i && now != start && now != end {
            check[i] = false;
            break;
        }

        if now == start {
            start = i;
        } else {
            end = i;
        }

        for j in 0..t[i].len() {
            if check[t[i][j]] {
                children[i] -= cnt[i][j];
            }
        }
        ret += children[start] * children[end];
        ret += children[start];
        ret += children[end];
    }

    if check.iter().all(|&m| m) {
        ret += n;
    }

    println!("{ret}")
}
