use proconio::*;

fn main() {
    input! {t: usize}

    for _ in 0..t {
        input! {k: i64, sx: i64, sy: i64, tx: i64, ty: i64}

        let dx = sx.div_euclid(k).abs_diff(tx.div_euclid(k));
        let dy = sy.div_euclid(k).abs_diff(ty.div_euclid(k));
        let dmin = dx.min(dy);

        let base = dmin as i64 * 2;
        if dx == dy {
            println!("{base}");
            continue;
        }
    }
}
