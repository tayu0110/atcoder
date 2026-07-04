use cpio::*;

fn main() {
    scan! {h: usize, w: usize, n: usize, p: [(u32, u32); n], q: usize, query: [(u8, u32); q]}

    let mut memo = [vec![vec![]; h + 1], vec![vec![]; w + 1]];
    for (i, (x, y)) in p.into_iter().enumerate() {
        memo[0][x as usize].push(i as u32);
        memo[1][y as usize].push(i as u32);
    }

    let mut removed = vec![false; n];
    for (ty, t) in query {
        let mut res = 0;
        for i in memo[ty as usize - 1][t as usize].drain(..) {
            if !removed[i as usize] {
                removed[i as usize] = true;
                res += 1;
            }
        }
        putln!(res);
    }
}
