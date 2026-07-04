use proconio::*;

// Return if `turn` can win
fn solve(
    turn: usize,
    n: usize,
    m: usize,
    white: usize,
    s: &[(usize, usize)],
    slen: usize,
    memo: &mut [[u8; 2]],
) -> bool {
    if memo[s.len()][white] < 2 {
        return memo[s.len()][white] == 1;
    }
    if s.is_empty() {
        let black = n - white;
        let pair = black * white % 2 + 2 - (m + slen - 1) % 2;
        // if pair % 2 == 0, win second, otherwise win first
        return pair % 2 != 0;
    }

    if !solve(turn + 1, n, m, (white + s[0].0) % 2, &s[1..], slen, memo) {
        memo[s.len()][white] = 1;
        return true;
    }
    if !solve(turn + 1, n, m, (white + s[0].1) % 2, &s[1..], slen, memo) {
        memo[s.len()][white] = 1;
        return true;
    }
    memo[s.len()][white] = 0;
    false
}

fn main() {
    input! {n: usize, m: usize, e: [(usize, usize); m]}

    let mut t = vec![vec![]; n];
    for (u, v) in e {
        t[u - 1].push(v - 1);
        t[v - 1].push(u - 1);
    }

    let mut color = vec![-1; n];
    let mut s = vec![];
    for i in 0..n {
        if color[i] >= 0 {
            continue;
        }
        let mut nt = vec![i];
        color[i] = 0;
        let (mut white, mut black) = (1, 0);
        while let Some(now) = nt.pop() {
            for &to in &t[now] {
                if color[to] < 0 {
                    color[to] = (color[now] + 1) % 2;
                    if color[to] == 0 {
                        white += 1;
                    } else {
                        black += 1;
                    }
                    nt.push(to);
                }
            }
        }
        s.push((white, black));
    }

    let mut memo = vec![[3; 2]; s.len() + 1];
    if solve(0, n, m, (s[0].0) % 2, &s[1..], s.len(), &mut memo[..]) {
        println!("Aoki")
    } else {
        println!("Takahashi")
    }
}
