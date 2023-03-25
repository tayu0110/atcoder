use proconio::input;

fn main() {
    input! {n: usize, m: usize, p: [(usize, usize); m]}

    let mut t = vec![vec![]; n];
    let mut ins = vec![0; n];
    for (x, y) in p {
        t[x - 1].push(y - 1);
        ins[y - 1] += 1;
    }

    let mut nt = ins
        .iter()
        .enumerate()
        .filter(|&(_, &v)| v == 0)
        .map(|(i, _)| i)
        .collect::<std::collections::VecDeque<_>>();
    let mut res = vec![0; n];
    while let Some(now) = nt.pop_front() {
        for &to in &t[now] {
            res[to] = std::cmp::max(res[to], res[now] + 1);
            ins[to] -= 1;

            if ins[to] == 0 {
                nt.push_back(to);
            }
        }
    }

    println!("{}", res.into_iter().max().unwrap())
}
