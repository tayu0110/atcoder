use proconio::*;

fn dfs(
    n: usize,
    h: usize,
    w: usize,
    r: usize,
    c: usize,
    now: usize,
    s: &[Vec<char>],
    t: &[char],
) -> bool {
    if now == n {
        return true;
    }
    let (nr, nc) = match t[now] {
        'L' => (r, c.wrapping_sub(1)),
        'R' => (r, c + 1),
        'U' => (r.wrapping_sub(1), c),
        'D' => (r + 1, c),
        _ => (0, 0),
    };

    if nr >= h || nc >= w || s[nr][nc] == '#' {
        false
    } else {
        dfs(n, h, w, nr, nc, now + 1, s, t)
    }
}

fn main() {
    input! {h: usize, w: usize, n: usize, t: marker::Chars, s: [marker::Chars; h]}

    let mut res = 0;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                continue;
            }
            res += dfs(n, h, w, i, j, 0, &s, &t) as usize;
        }
    }

    println!("{}", res);
}
