use proconio::*;

fn main() {
    input! {_: usize, _: usize, n: usize, p: [(usize, usize); n]}

    let mut res = 0;
    for v in p.windows(2) {
        let (x, y) = v[0];
        let (nx, ny) = v[1];
        if (x < nx) == (y < ny) {
            res += nx.abs_diff(x).max(ny.abs_diff(y));
        } else {
            res += nx.abs_diff(x) + ny.abs_diff(y);
        }
    }

    println!("{res}")
}
