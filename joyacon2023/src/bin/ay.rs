use proconio::input;

fn main() {
    input! {n: usize, k: usize, a: [usize; k], p: [(i64, i64); n]}

    let mut max = 0;
    for &(x, y) in &p {
        let mut min = std::i64::MAX;
        for &a in &a {
            let (nx, ny) = p[a - 1];

            min = std::cmp::min(min, (nx - x) * (nx - x) + (ny - y) * (ny - y));
        }

        max = std::cmp::max(max, min);
    }

    println!("{}", (max as f64).sqrt());
}
