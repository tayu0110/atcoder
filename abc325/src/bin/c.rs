use proconio::*;

fn dfs(r: usize, c: usize, checked: &mut Vec<Vec<bool>>, s: &Vec<Vec<u8>>) {
    let h = s.len();
    let w = s[0].len();
    checked[r][c] = true;
    for (dr, dc) in [
        (0, 1),
        (1, 0),
        (1, 1),
        (!0, 1),
        (1, !0),
        (!0, !0),
        (!0, 0),
        (0, !0),
    ] {
        let r = r.wrapping_add(dr);
        let c = c.wrapping_add(dc);
        if r < h && c < w && !checked[r][c] && s[r][c] == b'#' {
            dfs(r, c, checked, s);
        }
    }
}

fn main() {
    input! {h: usize, w: usize, s: [marker::Bytes; h]}

    let mut checked = vec![vec![false; w]; h];
    let mut res = 0;
    for i in 0..h {
        for j in 0..w {
            if !checked[i][j] && s[i][j] == b'#' {
                res += 1;
                dfs(i, j, &mut checked, &s);
            }
        }
    }

    println!("{res}");
}
