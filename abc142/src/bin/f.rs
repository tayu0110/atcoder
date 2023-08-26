use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, m: usize, e: [(usize, usize); m]}

    let mut t = vec![vec![]; n];
    for (a, b) in e {
        t[a - 1].push(b - 1);
    }

    let mut reached = vec![false; n];
    let mut state = vec![0; n];

    fn dfs(
        now: usize,
        reached: &mut [bool],
        state: &mut [u8],
        stack: &mut Vec<usize>,
        t: &Vec<Vec<usize>>,
    ) -> Option<usize> {
        state[now] = 1;
        for &to in &t[now] {
            if reached[to] {
                continue;
            }
            if state[to] == 1 {
                stack.push(now);
                return Some(to);
            }

            if let Some(res) = dfs(to, reached, state, stack, t) {
                if res == now {
                    stack.push(now);
                    return Some(std::usize::MAX);
                } else if res == std::usize::MAX {
                    return Some(res);
                } else {
                    stack.push(now);
                    return Some(res);
                }
            }
        }
        state[now] = 0;
        reached[now] = true;
        None
    }

    for i in 0..n {
        if reached[i] {
            continue;
        }

        let mut stack = vec![];
        if dfs(i, &mut reached, &mut state, &mut stack, &t).is_some() {
            stack.reverse();
            println!("{}", stack.len());
            println!("{}", stack.iter().map(|&s| s + 1).join("\n"));
            return;
        }
    }

    println!("-1")
}
