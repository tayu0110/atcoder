use proconio::*;

fn solve(n: usize, next: usize, init: &[[bool; 8]; 8], cur: &mut [[bool; 8]; 8]) -> usize {
    if next == n {
        return init
            .iter()
            .flatten()
            .zip(cur.iter().flatten())
            .filter(|(i, c)| i != c)
            .count()
            / 2;
    }

    let c = cur[next][..n]
        .iter()
        .rev()
        .fold(0, |s, v| (s << 1) | *v as u8);
    if c.count_ones() == 2 {
        solve(n, next + 1, init, cur)
    } else if c.count_ones() == 1 {
        let mut res = usize::MAX;
        for i in next + 1..n {
            if c & (1 << i) != 0 {
                continue;
            }

            if cur[i].iter().filter(|c| **c).count() == 2 {
                continue;
            }

            cur[i][next] = true;
            cur[next][i] = true;
            res = res.min(solve(n, next + 1, init, cur));
            cur[i][next] = false;
            cur[next][i] = false;
        }
        res
    } else {
        let mut res = usize::MAX;
        for i in next + 1..n {
            if cur[i].iter().filter(|c| **c).count() == 2 {
                continue;
            }
            cur[i][next] = true;
            cur[next][i] = true;
            for j in i + 1..n {
                if cur[j].iter().filter(|c| **c).count() == 2 {
                    continue;
                }
                cur[j][next] = true;
                cur[next][j] = true;
                res = res.min(solve(n, next + 1, init, cur));
                cur[j][next] = false;
                cur[next][j] = false;
            }
            cur[i][next] = false;
            cur[next][i] = false;
        }
        res
    }
}

fn main() {
    input! {n: usize, m: usize, e: [(usize, usize); m]}

    let mut edges = [[false; 8]; 8];
    for (a, b) in e {
        edges[a - 1][b - 1] = true;
        edges[b - 1][a - 1] = true;
    }

    println!("{}", solve(n, 0, &edges, &mut [[false; 8]; 8]));
}
