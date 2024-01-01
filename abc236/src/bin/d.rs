use proconio::*;

fn dfs(n: usize, rem: usize, now: usize, a: &Vec<Vec<usize>>) -> usize {
    if rem == (1 << n) - 1 {
        return now;
    }

    let mut res = 0;
    for i in 0..n {
        if rem & (1 << i) != 0 {
            continue;
        }

        for j in i + 1..n {
            if rem & (1 << j) == 0 {
                res = res.max(dfs(n, rem | (1 << i) | (1 << j), now ^ a[i][j], a));
            }
        }
        break;
    }

    res
}

fn main() {
    input! {n: usize}
    let mut a = vec![vec![0; 2 * n]; 2 * n];
    for i in 0..2 * n {
        for j in i + 1..2 * n {
            input! {na: usize}
            a[i][j] = na;
        }
    }

    println!("{}", dfs(2 * n, 0, 0, &a))
}
