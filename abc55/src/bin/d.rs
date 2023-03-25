use proconio::{marker::Chars, *};

fn solve(n: usize, a: usize, b: usize, s: &[char]) -> Option<Vec<usize>> {
    // 0: sheep, 1: wolf
    let mut state = vec![0; n];
    state[0] = a;
    state[1] = b;

    for i in 1..n - 1 {
        if state[i] == 0 {
            if s[i] == 'o' {
                state[i + 1] = state[i - 1];
            } else {
                state[i + 1] = 1 - state[i - 1];
            }
        } else {
            if s[i] == 'o' {
                state[i + 1] = 1 - state[i - 1];
            } else {
                state[i + 1] = state[i - 1];
            }
        }
    }

    if ((state[n - 1] == 0
        && ((s[n - 1] == 'o' && state[n - 2] == state[0])
            || (s[n - 1] == 'x' && state[n - 2] != state[0])))
        || (state[n - 1] == 1
            && ((s[n - 1] == 'o' && state[n - 2] != state[0])
                || (s[n - 1] == 'x' && state[n - 2] == state[0]))))
        && ((state[0] == 0
            && ((s[0] == 'o' && state[1] == state[n - 1])
                || (s[0] == 'x' && state[1] != state[n - 1])))
            || (state[0] == 1
                && ((s[0] == 'o' && state[1] != state[n - 1])
                    || (s[0] == 'x' && state[1] == state[n - 1]))))
    {
        Some(state)
    } else {
        None
    }
}

fn main() {
    input! {n: usize, s: Chars}

    for i in 0..2 {
        for j in 0..2 {
            if let Some(res) = solve(n, i, j, &s) {
                println!(
                    "{}",
                    res.into_iter()
                        .map(|v| if v == 0 { 'S' } else { 'W' })
                        .collect::<String>()
                );
                return;
            }
        }
    }

    println!("-1")
}
