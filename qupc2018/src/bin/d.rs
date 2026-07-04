use proconio::*;

fn main() {
    input! {n: usize, m: usize, l: usize, t: [i32; n], p: [(usize, i32); m], q: [(usize, i32); l]}

    let mut events = p
        .into_iter()
        .map(|(x, a)| (a, 0, x))
        .chain(q.into_iter().map(|(y, b)| (b, y, 0)))
        .collect::<Vec<_>>();
    events.sort_unstable();

    let mut stack = vec![vec![]; n + 1];
    stack[0] = vec![(-1, 0)];
    for (tm, from, to) in events {
        let pos = stack[from].partition_point(|s| s.0 < tm);
        if pos == 0 {
            continue;
        }
        let (_, max) = stack[from][pos - 1];
        stack[to].push((tm + t[from.max(to) - 1], max + 1));

        let len = stack[to].len();
        if len > 1 {
            let m = stack[to][len - 2].1;
            stack[to][len - 1].1 = m.max(max + 1);
        }
    }

    println!(
        "{}",
        stack.into_iter().flatten().map(|s| s.1).max().unwrap()
    );
}
