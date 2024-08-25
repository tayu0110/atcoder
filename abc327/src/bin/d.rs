use std::collections::VecDeque;

use proconio::*;

fn main() {
    input! {n: usize, m: usize, a: [usize; m], b: [usize; m]}

    let mut t = vec![vec![]; n];
    for (a, b) in a.into_iter().zip(b) {
        t[a - 1].push(b - 1);
        t[b - 1].push(a - 1);
    }

    let mut color = vec![2; n];
    let mut nt = VecDeque::new();
    for i in 0..n {
        if color[i] != 2 {
            continue;
        }

        color[i] = 0;
        nt.push_back(i);
        while let Some(now) = nt.pop_front() {
            for &to in &t[now] {
                if color[to] == 2 {
                    color[to] = (color[now] + 1) % 2;
                    nt.push_back(to);
                } else if color[to] != (color[now] + 1) % 2 {
                    println!("No");
                    return;
                }
            }
        }
    }

    println!("Yes")
}
