use proconio::*;

fn solve(s: &[char], last: char, now: usize, x: &mut usize) -> Option<char> {
    if now == *x {
        return Some(last);
    }

    if s[0].is_alphabetic() {
        if *x == 0 {
            return Some(s[0]);
        }

        if let Some(res) = solve(&s[1..], s[0], now.saturating_add(1), x) {
            Some(res)
        } else if *x == 0 {
            Some(s[0])
        } else {
            *x -= now;
            None
        }
    } else {
        let tm = s[0] as usize - b'0' as usize + 1;
        if now.saturating_mul(tm) >= *x {
            *x %= now;
            None
        } else {
            solve(&s[1..], last, now.saturating_mul(tm), x)
        }
    }
}

fn main() {
    input! {s: marker::Chars, mut x: usize}

    println!("{}", solve(&s, '!', 0, &mut x).unwrap())
}
