use proconio::*;

fn main() {
    input! {n: usize, p: [usize; n]}

    let mut max = 0;
    for i in 1..n {
        if p[0] <= p[i] {
            max = max.max(p[i] - p[0] + 1);
        }
    }

    println!("{}", max)
}
