use itertools::Itertools;
use math::MathInt;
use proconio::*;

fn main() {
    input! {n: usize, k: u64, p: [u32; n]}

    let mut used = vec![false; n];
    let mut res = vec![0; n];
    let mut stack = vec![];
    for i in 0..n {
        if !used[i] {
            let mut now = i as u32;
            stack.push(i as u32);
            used[i] = true;
            while p[now as usize] - 1 != stack[0] {
                stack.push(p[now as usize] - 1);
                now = p[now as usize] - 1;
                used[now as usize] = true;
            }
            let m = 2usize.pow_mod(k, stack.len());
            for (i, &r) in stack.iter().enumerate() {
                let to = (i + m) % stack.len();
                res[r as usize] = stack[to] + 1;
            }
            stack.clear();
        }
    }

    println!("{}", res.iter().join(" "))
}
