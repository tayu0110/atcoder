use proconio::input;

fn main() {
    input! {n: usize, p: [(i64, i64); n]}

    let mut max = 0;
    for i in 0..n {
        let (s, t) = p[i];
        for j in i + 1..n {
            let (v, w) = p[j];
            max = std::cmp::max(max, (v - s) * (v - s) + (w - t) * (w - t));
        }
    }

    println!("{}", (max as f64).sqrt());
}
