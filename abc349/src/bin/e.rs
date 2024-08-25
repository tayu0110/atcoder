use proconio::*;

// If game is finished, return Some(winner)
fn check(state: &[Vec<u8>]) -> Option<u8> {
    for i in 0..3 {
        if state[i][0] != 2 && state[i].iter().all(|&s| s == state[i][0]) {
            return Some(state[i][0]);
        }
    }

    for j in 0..3 {
        if state[0][j] != 2 && (0..3).all(|i| state[i][j] == state[0][j]) {
            return Some(state[0][j]);
        }
    }

    if state[0][0] != 2 && (0..3).all(|i| state[i][i] == state[0][0]) {
        return Some(state[0][0]);
    }

    if state[0][2] != 2 && (0..3).all(|i| state[i][2 - i] == state[0][2]) {
        return Some(state[0][2]);
    }

    None
}

// true: current player will win
// false: current player will lose
//
// 0: Takahashi
// 1: Aoki
// 2: undefined
fn dfs(turn: u8, a: &[Vec<i64>], pt: i64, pa: i64, state: &mut [Vec<u8>]) -> bool {
    if let Some(winner) = check(state) {
        return turn == winner;
    }

    if state.iter().flatten().all(|&s| s < 2) {
        let mut p = [0; 2];
        for i in 0..3 {
            for j in 0..3 {
                p[state[i][j] as usize] += a[i][j];
            }
        }

        return if turn == 0 { p[0] > p[1] } else { p[1] > p[0] };
    }

    for i in 0..3 {
        for j in 0..3 {
            if state[i][j] == 2 {
                state[i][j] = turn;
                let (pt, pa) = if turn == 0 {
                    (pt + a[i][j], pa)
                } else {
                    (pt, pa + a[i][j])
                };
                if !dfs(turn ^ 1, a, pt, pa, state) {
                    state[i][j] = 2;
                    return true;
                }
                state[i][j] = 2;
            }
        }
    }

    false
}

fn main() {
    input! {a: [[i64; 3]; 3]}

    let mut state = vec![vec![2; 3]; 3];
    if dfs(0, &a, 0, 0, &mut state) {
        println!("Takahashi")
    } else {
        println!("Aoki")
    }
}
