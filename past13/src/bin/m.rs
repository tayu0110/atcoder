use proconio::*;

fn main() {
    input! {n: usize, e: [(u32, u32); n - 1]}

    let mut t = vec![vec![]; n];
    for (i, (u, v)) in e.into_iter().enumerate() {
        t[u as usize - 1].push((v - 1, i as u32));
        t[v as usize - 1].push((u - 1, i as u32));
    }

    let mut res = 0usize;
    let mut stack = Vec::with_capacity(n);
    stack.push((0, 0, u32::MAX, 0));
    while let Some((now, par, min, max)) = stack.pop() {
        if min > max {
            res += (n - 1) * n / 2;
        } else {
            res += (min as usize + 1) * (n - 1 - max as usize);
        }

        stack.extend(t[now as usize].iter().filter_map(|&(to, index)| {
            (to != par).then_some((to, now, min.min(index), max.max(index)))
        }));
    }

    println!("{res}");
}
