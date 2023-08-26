use proconio::*;

fn main() {
    input! {n: usize, m: usize, p: [(i64, i64); n], q: [(i64, i64); m]}

    for (a, b) in p {
        let mut res = 0;
        let mut dist = std::i64::MAX;
        for (i, &(c, d)) in q.iter().enumerate() {
            if dist > (a - c).abs() + (b - d).abs() {
                res = i + 1;
                dist = (a - c).abs() + (b - d).abs();
            }
        }

        println!("{}", res)
    }
}
