use proconio::*;

fn main() {
    input! {n: usize, m: usize, a: [[usize; m]; n], b: [[usize; m]; n]}

    let mut now = 0;
    for i in 0..n {
        let mut min = usize::MAX;
        for j in 0..m {
            let t = (now + a[i][j] - 1) / a[i][j] * a[i][j];
            min = min.min(t + b[i][j]);
        }
        now = min;
    }

    println!("{now}")
}
