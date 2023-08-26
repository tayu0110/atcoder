use proconio::*;

fn dfs(now: usize, par: usize, memo: &mut Vec<Vec<usize>>, t: &Vec<Vec<usize>>) -> usize {
    let mut res = 1;
    for &to in &t[now] {
        if to == par {
            memo[now].push(std::usize::MAX);
        } else {
            let r = dfs(to, now, memo, t);
            res += r;
            memo[now].push(r);
        }
    }
    res
}

fn dfs2(now: usize, prp: usize, memo: &mut Vec<Vec<usize>>, t: &Vec<Vec<usize>>) {
    let mut sum = 1;
    let mut par = std::usize::MAX;
    for i in 0..t[now].len() {
        if memo[now][i] == std::usize::MAX {
            memo[now][i] = prp;
            par = i;
        }
        sum += memo[now][i];
    }

    for i in 0..t[now].len() {
        if i == par {
            continue;
        }

        dfs2(t[now][i], sum - memo[now][i], memo, t);
    }
}

fn main() {
    input! {n: usize, e: [(usize, usize); n-1]}

    let mut t = vec![vec![]; n];
    for (a, b) in e {
        t[a - 1].push(b - 1);
        t[b - 1].push(a - 1);
    }

    let mut memo = vec![vec![]; n];
    dfs(0, 0, &mut memo, &t);
    dfs2(0, 0, &mut memo, &t);

    let mut res = 0usize;
    for memo in memo {
        if memo.len() < 3 {
            continue;
        }

        let mut cumk = memo.clone();
        for i in (0..memo.len() - 1).rev() {
            cumk[i] += cumk[i + 1];
        }
        let mut cumj = vec![0; memo.len() - 1];
        for j in 0..memo.len() - 1 {
            cumj[j] = memo[j] * cumk[j + 1];
        }
        for j in (0..memo.len() - 2).rev() {
            cumj[j] += cumj[j + 1];
        }
        let mut sum = 0;
        for i in 0..memo.len() - 2 {
            sum += memo[i] * cumj[i + 1];
        }
        res += sum;
    }

    println!("{}", res)
}
