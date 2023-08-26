use std::collections::HashSet;

use proconio::*;

fn succ(now: i32, len: i32, d: i32) -> i32 {
    (now + len + d) % len
}

fn dfs(
    y: i32,
    x: i32,
    mem: i32,
    dir: (i32, i32),
    s: &Vec<Vec<char>>,
    memo: &mut HashSet<(i32, i32, i32, (i32, i32))>,
) -> bool {
    let r = s.len() as i32;
    let c = s[0].len() as i32;
    if memo.contains(&(y, x, mem, dir)) {
        return false;
    }
    memo.insert((y, x, mem, dir));
    match s[y as usize][x as usize] {
        '<' => dfs(y, succ(x, c, -1), mem, (0, -1), s, memo),
        '>' => dfs(y, succ(x, c, 1), mem, (0, 1), s, memo),
        '^' => dfs(succ(y, r, -1), x, mem, (-1, 0), s, memo),
        'v' => dfs(succ(y, r, 1), x, mem, (1, 0), s, memo),
        '_' => {
            if mem == 0 {
                dfs(y, succ(x, c, 1), mem, (0, 1), s, memo)
            } else {
                dfs(y, succ(x, c, -1), mem, (0, -1), s, memo)
            }
        }
        '|' => {
            if mem == 0 {
                dfs(succ(y, r, 1), x, mem, (1, 0), s, memo)
            } else {
                dfs(succ(y, r, -1), x, mem, (-1, 0), s, memo)
            }
        }
        '?' => vec![(0, 1), (1, 0), (0, -1), (-1, 0)]
            .into_iter()
            .map(|(dy, dx)| dfs(succ(y, r, dy), succ(x, c, dx), mem, (dy, dx), s, memo))
            .fold(false, |s, v| s | v),
        '.' => dfs(succ(y, r, dir.0), succ(x, c, dir.1), mem, dir, s, memo),
        '@' => true,
        '+' => dfs(
            succ(y, r, dir.0),
            succ(x, c, dir.1),
            (mem + 1) % 16,
            dir,
            s,
            memo,
        ),
        '-' => dfs(
            succ(y, r, dir.0),
            succ(x, c, dir.1),
            (mem + 16 - 1) % 16,
            dir,
            s,
            memo,
        ),
        n => dfs(
            succ(y, r, dir.0),
            succ(x, c, dir.1),
            n as i32 - b'0' as i32,
            dir,
            s,
            memo,
        ),
    }
}

fn main() {
    input! {r: usize, _: usize, s: [marker::Chars; r]}

    let mut memo = HashSet::new();
    if dfs(0, 0, 0, (0, 1), &s, &mut memo) {
        println!("YES")
    } else {
        println!("NO")
    }
}
