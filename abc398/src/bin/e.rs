use std::{collections::HashSet, io::Write};

use proconio::*;

fn main() {
    input_interactive!(n: usize, e: [(usize, usize); n - 1]);

    let mut t = vec![vec![]; n];
    for &(u, v) in &e {
        t[u - 1].push(v - 1);
        t[v - 1].push(u - 1);
    }

    let mut color = vec![-1; n];
    let mut nt = vec![0];
    color[0] = 0;
    while let Some(now) = nt.pop() {
        for &to in &t[now] {
            if color[to] < 0 {
                color[to] = (color[now] + 1) % 2;
                nt.push(to);
            }
        }
    }

    let mut pair = HashSet::new();
    for i in 0..n {
        for j in i + 1..n {
            if color[i] != color[j] {
                pair.insert((i, j));
            }
        }
        for &to in &t[i] {
            if color[i] != color[to] {
                pair.remove(&(i.min(to), i.max(to)));
            }
        }
    }

    if pair.len() % 2 == 0 {
        println!("Second");
        std::io::stdout().flush().unwrap();
        input_interactive!(i: i32, j: i32);
        if i >= 0 {
            let (i, j) = (i - 1, j - 1);
            pair.remove(&(i.min(j) as usize, i.max(j) as usize));
        }
    } else {
        println!("First");
        std::io::stdout().flush().unwrap();
    }

    loop {
        let &(i, j) = pair.iter().next().unwrap();
        pair.remove(&(i, j));
        println!("{} {}", i + 1, j + 1);
        std::io::stdout().flush().unwrap();

        input_interactive!(i: i32, j: i32);
        if i < 0 {
            break;
        }

        let (i, j) = (i - 1, j - 1);
        pair.remove(&(i.min(j) as usize, i.max(j) as usize));
    }
}
