use proconio::{input, marker::Chars};

fn solve(n: usize, k: usize, mut s: Vec<char>) -> bool {
    let m = n + k;
    s.resize(m, '?');

    for i in 0..m {
        if s[m - i - 1] == '?' {
            s[m - i - 1] = s[i];
        }

        if s[i] != s[m - i - 1] {
            return false;
        }
    }

    let t = [&s[n..m], &s[0..n]].concat();

    for i in 0..m {
        if t[i] != t[m - i - 1] {
            return false;
        }
    }
    true
}

fn main() {
    input! {t: usize}

    for _ in 0..t {
        input! {n: usize, k: usize, mut s: Chars}

        if k <= n {
            if solve(n, k, s) {
                println!("Yes")
            } else {
                println!("No")
            }
        } else {
            let v = (k + n) % (2 * n);

            let s2 = s
                .iter()
                .chain(s.iter().rev()).copied()
                .collect::<Vec<char>>();
            if solve(n * 2, v, s2) {
                println!("Yes")
            } else {
                println!("No")
            }
        }
    }
}
