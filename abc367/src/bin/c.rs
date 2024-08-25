use itertools::Itertools;
use proconio::*;

fn dfs(n: usize, k: usize, r: &[usize], stack: &mut Vec<usize>) {
    if stack.len() == n {
        let sum = stack.iter().sum::<usize>();
        if sum % k == 0 {
            println!("{}", stack.iter().join(" "));
        }
        return;
    }

    let now = stack.len();
    for i in 1..=r[now] {
        stack.push(i);
        dfs(n, k, r, stack);
        stack.pop();
    }
}

fn main() {
    input! {n: usize, k: usize, r: [usize; n]}
    dfs(n, k, &r, &mut vec![])
}
