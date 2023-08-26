use proconio::*;

fn dfs(
    now: usize,
    n: usize,
    m: usize,
    stack: &mut Vec<usize>,
    p: &[(usize, usize, usize, usize)],
) -> usize {
    if stack.len() == n {
        let mut res = 0;
        for &(a, b, c, d) in p {
            if stack[b - 1] - stack[a - 1] == c {
                res += d;
            }
        }
        return res;
    }

    let mut res = 0;
    for next in now..=m {
        stack.push(next);
        res = res.max(dfs(next, n, m, stack, p));
        stack.pop().unwrap();
    }

    res
}

fn main() {
    input! {n: usize, m: usize, q: usize, p: [(usize, usize, usize, usize); q]}

    let mut stack = vec![];
    println!("{}", dfs(1, n, m, &mut stack, &p));
}
