use itertools::Itertools;
use num::Integer;
use proconio::*;

fn main() {
    input! {t: usize}

    let mut buf = vec![];
    for _ in 0..t {
        input! {px: i64, py: i64, qx: i64, qy: i64, rx: i64, ry: i64, sx: i64, sy: i64}

        let l = |p: i64, q: i64| -> i64 { -2 * (p - q) };
        let m = |p: i64, q: i64| -> i64 { -2 * (p - q) };
        let n =
            |px: i64, qx: i64, py: i64, qy: i64| -> i64 { px * px - qx * qx + py * py - qy * qy };

        let mut pl = l(px, py);
        let mut pm = m(px, py);
        let mut pn = n(px, qx, py, qy);

        let mut rl = l(rx, ry);
        let mut rm = m(sx, sy);
        let mut rn = n(rx, ry, sx, sy);

        let mut pg = pl.gcd(&pm);
        if pn != 0 {
            pg = pg.gcd(&pn);
        }
        if pg != 0 {
            pl /= pg;
            pm /= pg;
            pn /= pg;
        }

        let mut rg = rl.gcd(&rm);
        if rg != 0 {
            rg = rg.gcd(&rn);
        }
        if rg != 0 {
            rl /= rg;
            rm /= rg;
            rn /= rg;
        }

        eprintln!("pl: {pl}, pm: {pm}, pn: {pn}, rl: {rl}, rm: {rm}, rn: {rn}");

        match (pl == rl, pm == rm) {
            (true, true) => {
                if pn == rn {
                    buf.push("Yes")
                } else {
                    buf.push("No");
                }
            }
            _ => buf.push("Yes"),
        }
    }

    println!("{}", buf.iter().join("\n"))
}
