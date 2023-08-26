use proconio::*;

fn main() {
    input! {n: usize, mut p: [(i64, i64); n]}

    p.sort_by(|&(lx, ly), &(rx, ry)| (ly * (rx - 1)).cmp(&(ry * (lx - 1))));

    let mut px = 2;
    let mut py = 0;
    let mut res = 0;

    for (x, y) in p {
        if py * x <= (y - 1) * (px - 1) {
            res += 1;
            px = x;
            py = y;
        }
    }

    println!("{}", res)
}
