use proconio::*;

fn main() {
    input! {n: usize, p: [(usize, usize); n], q: usize}

    let mut con = vec![0; n - 1];
    for (i, v) in p.windows(2).enumerate() {
        let (l, u) = v[0];
        let (nl, nu) = v[1];
    }

    for _ in 0..q {
        input! {mut sx: usize, mut sy: usize, mut tx: usize, mut ty: usize}

        if tx < sx {
            (sx, sy, tx, ty) = (tx, ty, sx, sy);
        }

        if sx == tx {
            println!("{}", sy.abs_diff(ty));
            continue;
        }
    }
}
