use proconio::*;

fn main() {
    input! {n: usize, mut p: [(f64, f64); n]}
    p.insert(0, (0.0, 0.0));
    p.push((0.0, 0.0));
    println!(
        "{}",
        p.windows(2)
            .map(|v| {
                let (x, y) = v[0];
                let (nx, ny) = v[1];
                (nx - x).hypot(ny - y)
            })
            .sum::<f64>()
    );
}
