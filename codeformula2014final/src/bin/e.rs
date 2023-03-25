use proconio::{input, marker::Chars};

fn solve(n: usize, k: usize, s: &[char], fib: &[usize]) -> bool {
    if s.len() == 1 {
        return k == 0 && ((n == 1 && s[0] == 'b') || (n == 2 && s[0] == 'a'));
    }

    if k % 2 == 1 && n % 2 == 1 && s[0] == 'a' {
        return false;
    }

    if k % 2 == 0 {
        solve(n - 1, k / 2, &s[..fib[n - 1]], fib) && solve(n - 2, k / 4, &s[fib[n - 1]..], fib)
    } else {
        solve(n - 2, k / 4, &s[..fib[n - 2]], fib) && solve(n - 1, k / 2, &s[fib[n - 2]..], fib)
    }
}

fn main() {
    input! {s: Chars}

    if s.len() == 1 {
        if s[0] == 'b' {
            println!("1 0");
        } else {
            println!("2 0");
        }
        return;
    }

    let mut fib = vec![0usize, 1, 1];
    for i in 3..30 {
        fib.push(fib[i - 1] + fib[i - 2]);
        if fib[i] >= s.len() {
            break;
        }
    }

    let n = fib.len() - 1;
    let k_max = 1 << (n - 2);

    for k in 0..k_max {
        if solve(n, k, &s, &fib) {
            println!("{} {}", n, k);
            return;
        }
    }
}
