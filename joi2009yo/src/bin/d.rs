use proconio::*;

fn dfs(r: usize, c: usize, cnt: usize, res: &mut usize, a: &mut [Vec<u8>]) {
    if a[r][c] == 0 {
        *res = cnt.max(*res);
        return;
    }

    let (n, m) = (a.len(), a[0].len());
    a[r][c] = 0;
    for (dr, dc) in [(0, 1), (1, 0), (0, !0), (!0, 0)] {
        let nr = r.wrapping_add(dr);
        let nc = c.wrapping_add(dc);
        if nr < n && nc < m {
            dfs(nr, nc, cnt + 1, res, a);
        }
    }
    a[r][c] = 1;
}

fn main() {
    input! {m: usize, n: usize, mut a: [[u8; m]; n]}

    let mut res = 0;
    for i in 0..n {
        for j in 0..m {
            dfs(i, j, 0, &mut res, &mut a);
        }
    }
    println!("{res}")
}
