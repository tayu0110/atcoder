use itertools::Itertools;
use proconio::*;

#[fastout]
fn main() {
    input! {mut h: usize, mut w: usize, q: usize, mut e: [(usize, usize, char); q]}
    let mut transposed = false;
    if h < w {
        (h, w) = (w, h);
        e.iter_mut().for_each(|c| (c.0, c.1) = (c.1, c.0));
        transposed = true;
    }

    let mut height = vec![0; w];
    let mut ret = vec![vec!['A'; w]; h];
    for &(r, c, x) in e.iter().rev() {
        let (mut lh, mut rh) = (0, w);
        while rh - lh > 1 {
            let m = (rh + lh) / 2;
            if height[m] > r {
                lh = m;
            } else {
                rh = m;
            }
        }
        if height[lh] > r {
            lh += 1;
        }
        for c in lh..c {
            if height[c] < r {
                for r in height[c]..r {
                    ret[r][c] = x;
                }
                height[c] = r;
            }
        }
    }

    if transposed {
        let mut tmp = vec![vec!['A'; h]; w];
        for i in 0..h {
            for j in 0..w {
                tmp[j][i] = ret[i][j];
            }
        }
        ret = tmp;
    }
    for row in ret {
        println!("{}", row.iter().join(""))
    }
}
