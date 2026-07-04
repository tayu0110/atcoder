use proconio::*;

fn solve(level: usize, column: usize, t: &[Vec<u8>], memo: &mut [Vec<usize>]) {
    if level == t.len() - 1 {
        memo[level][column] = 1;
        return;
    }
    let now = t[level][column];

    let mut cnt = 0;
    let mut buf = vec![];
    for i in 0..3 {
        let next = column * 3 + i;
        solve(level + 1, next, t, memo);
        if now == t[level + 1][next] {
            cnt += 1;
            buf.push(memo[level + 1][next]);
        }
    }

    buf.sort();
    memo[level][column] = buf[..cnt - 1].iter().sum::<usize>();
}

fn main() {
    input! {_: usize, a: marker::Bytes}
    let a = a.into_iter().map(|a| a - b'0').collect::<Vec<_>>();

    let mut memo = vec![vec![0usize; a.len()]];
    let mut t = vec![a];
    while t.last().unwrap().len() > 1 {
        let last = t.last().unwrap();
        let new = last
            .chunks_exact(3)
            .map(|v| (v.iter().sum::<u8>() > 1) as u8)
            .collect::<Vec<_>>();
        memo.push(vec![0; new.len()]);
        t.push(new);
    }

    memo.reverse();
    t.reverse();
    solve(0, 0, &t, &mut memo);
    println!("{}", memo[0][0])
}
