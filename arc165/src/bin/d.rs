use proconio::*;

fn dfs(now: usize, state: &mut Vec<Vec<u8>>, p: &[(usize, usize, usize, usize)]) -> bool {
    if now == p.len() {
        return true;
    }

    let (a, b, c, d) = p[now];
    let len = (b - a).min(d - c);
    let mut save = vec![];
    for i in 0..len {
        let (na, nc) = (a + i, c + i);
        save.push((state[na][nc], state[nc][na]));
        if state[na][nc] == 0 || state[na][nc] == u8::MAX {
            let old = state[na][nc];
            let old_rev = state[nc][na];
            state[na][nc] = 0;
            state[nc][na] = 2;
            let f = dfs(now + 1, state, p);
            if f {
                return true;
            }

            state[na][nc] = old;
            state[nc][na] = old_rev;
        } else {
            for j in 0..=i {
                let (na, nc) = (a + j, c + j);
                (state[na][nc], state[nc][na]) = save[j];
            }
            return false;
        }

        if state[na][nc] != 1 && state[na][nc] != u8::MAX {
            for j in 0..=i {
                let (na, nc) = (a + j, c + j);
                (state[na][nc], state[nc][na]) = save[j];
            }
            return false;
        }

        state[na][nc] = 1;
        state[nc][na] = 1;
    }

    if b - a < d - c {
        let f = dfs(now + 1, state, p);
        if f {
            return true;
        }
    }

    for i in 0..len {
        let (na, nc) = (a + i, c + i);
        (state[na][nc], state[nc][na]) = save[i];
    }
    false
}

fn main() {
    input! {n: usize, m: usize, mut p: [(usize, usize, usize, usize); m]}
    p.iter_mut().for_each(|v| {
        v.0 -= 1;
        v.2 -= 1;
    });

    // 0: i < j, 1: i == j, 2: i > j, u8::MAX: not decided
    let mut state = vec![vec![u8::MAX; n]; n];
    if dfs(0, &mut state, &p) {
        println!("Yes")
    } else {
        println!("No")
    }
}
