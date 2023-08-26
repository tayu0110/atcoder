use std::collections::HashMap;

use proconio::*;

fn solve(
    now: usize,
    a: usize,
    b: usize,
    c: usize,
    d: usize,
    s: &[char],
    memo: &mut HashMap<(usize, usize, usize, usize), bool>,
) -> bool {
    let len = s.len();
    if now == len {
        return true;
    }

    if let Some(f) = memo.get(&(a, b, c, d)) {
        return *f;
    }

    if now + 1 < len {
        if s[now] == 'o' && s[now + 1] == 'x' && a > 0 {
            let res = solve(now + 2, a - 1, b, c, d, s, memo);
            if res {
                return true;
            }
        } else if s[now] == 'x' && s[now + 1] == 'o' && b > 0 {
            let res = solve(now + 2, a, b - 1, c, d, s, memo);
            if res {
                return true;
            }
        }
    }

    if s[now] == 'o' && c > 0 {
        let res = solve(now + 1, a, b, c - 1, d, s, memo);
        if res {
            return true;
        }
    } else if s[now] == 'x' && d > 0 {
        let res = solve(now + 1, a, b, c, d - 1, s, memo);
        if res {
            return true;
        }
    }

    memo.insert((a, b, c, d), false);
    false
}

fn main() {
    input! {_: usize, s: marker::Chars, a: usize, b: usize, c: usize, d: usize}

    if s.iter().filter(|&&c| c == 'o').count() != a + b + c {
        println!("No");
        return;
    }

    let mut memo = HashMap::new();
    if solve(0, a, b, c, d, &s, &mut memo) {
        println!("Yes")
    } else {
        println!("No")
    }
}
