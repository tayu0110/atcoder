use proconio::*;

fn dfs(
    now: usize,
    par: usize,
    has_photo: &[bool],
    memo: &mut Vec<Vec<u32>>,
    t: &Vec<Vec<usize>>,
) -> bool {
    let mut res = has_photo[now];
    for &to in &t[now] {
        if to == par {
            memo[now].push(std::u32::MAX);
            continue;
        }

        if dfs(to, now, has_photo, memo, t) {
            res = true;
            memo[now].push(1);
        } else {
            memo[now].push(0);
        }
    }
    res
}

fn dfs2(now: usize, p: u32, has_photo: &[bool], memo: &mut Vec<Vec<u32>>, t: &Vec<Vec<usize>>) {
    let mut res = has_photo[now] as u32;
    let mut par = now;
    for (i, &to) in t[now].iter().enumerate() {
        if memo[now][i] == std::u32::MAX {
            par = to;
            memo[now][i] = (p > 0) as u32;
        }

        res += (memo[now][i] > 0) as u32;
    }

    for (i, &to) in t[now].iter().enumerate() {
        if to != par {
            dfs2(to, res - memo[now][i], has_photo, memo, t);
        }
    }
}

fn main() {
    input! {n: usize, m: usize, e: [(usize, usize); n-1], c: [usize; m]}

    let mut has_photo = vec![false; n];
    for c in c {
        has_photo[c - 1] = true;
    }

    let mut t = vec![vec![]; n];
    for (a, b) in e {
        t[a - 1].push(b - 1);
        t[b - 1].push(a - 1);
    }

    let mut memo = vec![vec![]; n];
    dfs(0, 0, &has_photo, &mut memo, &t);
    dfs2(0, 0, &has_photo, &mut memo, &t);

    if memo
        .iter()
        .any(|v| v.into_iter().filter(|&&v| v > 0).count() >= 3)
    {
        println!("trumpet")
    } else {
        println!("Yes")
    }
}
