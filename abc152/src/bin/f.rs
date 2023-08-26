use proconio::*;

fn dfs(
    now: usize,
    par: usize,
    tar: usize,
    stack: &mut Vec<usize>,
    t: &Vec<Vec<(usize, usize)>>,
) -> bool {
    if now == tar {
        return true;
    }

    for &(to, id) in &t[now] {
        if to != par {
            if dfs(to, now, tar, stack, t) {
                stack.push(id);
                return true;
            }
        }
    }
    false
}

fn solve(
    from: usize,
    to: usize,
    clause: &[(usize, usize)],
    t: &Vec<Vec<(usize, usize)>>,
) -> Vec<usize> {
    let n = to - from;
    let m = clause.len();
    let mut res = vec![0; 1 << m];
    for i in 0..1 << n {
        let mut r = 0u32;
        for (j, (u, v)) in clause.iter().map(|&(u, v)| (u - 1, v - 1)).enumerate() {
            let mut stack = vec![];
            dfs(u, u, v, &mut stack, t);
            for s in stack {
                if from <= s && s < to && (1 << (s - from)) & i != 0 {
                    r |= 1 << j;
                    break;
                }
            }
        }

        res[r as usize] += 1;
    }
    res
}

fn main() {
    input! {n: usize, e: [(usize, usize); n-1], m: usize, p: [(usize, usize); m]}

    let mut t = vec![vec![]; n];
    for (i, (a, b)) in e.into_iter().map(|(a, b)| (a - 1, b - 1)).enumerate() {
        t[a].push((b, i));
        t[b].push((a, i));
    }

    let f = solve(0, n / 2, &p, &t);
    let b = solve(n / 2, n, &p, &t);

    let mut res = 0;
    for i in 0..1 << m {
        if f[i] == 0 {
            continue;
        }
        let rem = ((1 << m) - 1) ^ i;
        let mut now = i;
        while now > 0 {
            let tar = now | rem;
            res += f[i] * b[tar];
            now = (now - 1) & i;
        }

        res += f[i] * b[rem];
    }

    println!("{}", res / 2)
}
