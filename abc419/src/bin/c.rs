use proconio::*;

fn solve(mut x: Vec<usize>) -> usize {
    let n = x.len();
    x.sort_unstable();
    (x[n - 1] - x[0] + 1) / 2
}

fn main() {
    input! {n: usize, p: [(usize, usize); n]}

    if n == 1 {
        println!("0");
        return;
    }

    let (x, y) = p
        .into_iter()
        .unzip::<usize, usize, Vec<usize>, Vec<usize>>();
    println!("{}", solve(x).max(solve(y)));
}
