use proconio::*;

fn main() {
    input! {n: usize, p: [(usize, usize); n]}

    let mut ret = vec![false; n + 1];
    let mut t = vec![vec![]; n + 1];
    let mut nt = vec![];
    for (i, (a, b)) in p.into_iter().enumerate() {
        if a == 0 && b == 0 {
            ret[i + 1] = true;
            nt.push(i + 1);
            continue;
        }

        t[a].push(i + 1);
        t[b].push(i + 1);
    }

    while let Some(now) = nt.pop() {
        for &to in &t[now] {
            if !ret[to] {
                ret[to] = true;
                nt.push(to);
            }
        }
    }

    println!("{}", ret.iter().filter(|&&ret| ret).count())
}
