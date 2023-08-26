use proconio::*;

fn rec(r: usize, c: usize, s: &Vec<Vec<u8>>, memo: &mut Vec<Vec<u8>>) -> u8 {
    if r == s.len() - 1 && c == s[0].len() - 1 {
        memo[r][c] = 1;
        return 1;
    }

    if memo[r][c] > 0 {
        return memo[r][c];
    }

    let next = (s[r][c] + 1) % 5;
    memo[r][c] = 3;

    let mut res = 2;
    for (dr, dc) in vec![(0, 1), (1, 0), (0, !0), (!0, 0)] {
        let (nr, nc) = (r.wrapping_add(dr), c.wrapping_add(dc));
        if nr >= s.len() || nc >= s[0].len() {
            continue;
        }
        if s[nr][nc] != next {
            continue;
        }
        if memo[nr][nc] == 3 {
            continue;
        }
        res = res.min(rec(nr, nc, s, memo));
    }
    memo[r][c] = res;
    res
}

fn main() {
    input! {h: usize, w: usize, mut s: [marker::Chars; h]}

    let mut ns = vec![vec![std::u8::MAX; w]; h];
    for i in 0..h {
        for j in 0..w {
            ns[i][j] = match s[i][j] {
                's' => 0,
                'n' => 1,
                'u' => 2,
                'k' => 3,
                'e' => 4,
                _ => std::u8::MAX,
            };
        }
    }

    if ns[0][0] != 0 {
        println!("No");
        return;
    }

    // 0: undecided, 1: true, 2: false, 3: in progress
    let mut memo = vec![vec![0u8; w]; h];
    if rec(0, 0, &ns, &mut memo) == 1 {
        println!("Yes");
    } else {
        println!("No");
    }
}
