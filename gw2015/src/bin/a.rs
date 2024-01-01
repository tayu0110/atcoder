use itertools::Itertools;

fn dfs(now: usize, sum: i32, t: &[[i32; 3]], res: &mut Vec<i32>) {
    if now == t.len() {
        res.push(sum);
        return;
    }

    for &nt in &t[now] {
        dfs(now + 1, sum + nt, t, res);
    }
}

fn main() {
    let t = vec![
        [0, 0, 25],
        [0, 0, 39],
        [0, 0, 51],
        [0, 0, 76],
        [0, 0, 163],
        [0, 0, 111],
        [0, 58, 136],
        [0, 0, 128],
        [0, 0, 133],
        [0, 0, 138],
    ];

    let mut res = vec![];
    dfs(0, 0, &t, &mut res);
    res.sort_unstable();
    res.dedup();
    println!("{}", res.iter().join("\n"))
}
