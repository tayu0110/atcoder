use itertools::Itertools;
use proconio::*;

fn dfs(now: usize, reached: &mut [bool], stack: &mut Vec<usize>, t: &Vec<Vec<usize>>) {
    reached[now] = true;
    stack.push(now);
    for &to in &t[now] {
        if reached[to] {
            return;
        }
        dfs(to, reached, stack, t);
    }
}

fn main() {
    input! {n: usize, mut a: [usize; n]}
    a.iter_mut().for_each(|v| *v -= 1);

    let mut now = 0;
    for _ in 0..n {
        now = a[now];
    }

    let mut t = vec![vec![]; n];
    for (i, a) in a.into_iter().enumerate() {
        t[i].push(a);
    }

    let mut reached = vec![false; n];
    let mut stack = vec![];
    dfs(now, &mut reached, &mut stack, &t);

    println!("{}", stack.len());
    println!("{}", stack.into_iter().map(|v| v + 1).join(" "))
}
