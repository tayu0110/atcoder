use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, p: [(f64, f64); n]}

    let mut sum = 0.0;
    for v in (0..n).permutations(n) {
        let mut np = vec![];
        for v in v {
            np.push(p[v]);
        }

        for v in np.windows(2) {
            let (x, y) = v[0];
            let (nx, ny) = v[1];
            sum += (nx - x).hypot(ny - y);
        }
    }

    for i in 1..=n {
        sum /= i as f64;
    }

    println!("{}", sum)
}
