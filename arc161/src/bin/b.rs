use proconio::*;

fn rec(now: usize, b: i32, n: usize) -> Option<usize> {
    if now.count_ones() == 3 && now <= n {
        return Some(now);
    }

    if b < 0 {
        return None;
    }

    if now + (1 << b) <= n {
        if let Some(res) = rec(now + (1 << b), b - 1, n) {
            return Some(res);
        }
    }

    if let Some(res) = rec(now, b - 1, n) {
        return Some(res);
    }

    None
}

fn main() {
    input! {t: usize}

    for _ in 0..t {
        input! {n: usize}

        if let Some(res) = rec(0, 62, n) {
            println!("{}", res)
        } else {
            println!("-1")
        }
    }
}
