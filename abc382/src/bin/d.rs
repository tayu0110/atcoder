use std::fmt::Write;

use itertools::Itertools;
use proconio::*;

fn dfs(n: usize, m: u16, stack: &mut Vec<u16>, out: &mut String) {
    if stack.len() == n {
        writeln!(out, "{}", stack.iter().join(" ")).ok();
        return;
    }

    let rem = n - stack.len();
    let last = *stack.last().unwrap();
    if last + 10 * rem as u16 > m {
        return;
    }

    for next in last + 10..=m {
        stack.push(next);
        dfs(n, m, stack, out);
        stack.pop();
    }
}

fn main() {
    input! {n: usize, m: u16}

    let mut res = String::new();
    for first in 1..=m {
        dfs(n, m, &mut vec![first], &mut res);
    }

    println!("{}", res.lines().count());
    print!("{}", res);
}
