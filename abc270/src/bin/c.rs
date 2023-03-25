#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn dfs(now: usize, par: usize, target: usize, res: &mut Vec<usize>, t: &Vec<Vec<usize>>) -> bool {
    if now == target {
        res.push(now);
        return true;
    }

    for to in &t[now] {
        if *to != par {
            if dfs(*to, now, target, res, t) {
                res.push(now);
                return true;
            }
        }
    }

    false
}

fn main() {
    input! {n: usize, x: usize, y: usize, p: [(usize, usize); n-1]}

    let mut t = vec![vec![]; n];
    for (u, v) in p {
        t[u-1].push(v-1);
        t[v-1].push(u-1);
    }

    let mut res = vec![];
    dfs(x-1, std::usize::MAX, y-1, &mut res, &t);

    res.reverse();

    for i in 0..res.len() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", res[i]+1);
    }
    println!("");
}
