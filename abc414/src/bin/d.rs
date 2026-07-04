use proconio::*;

fn main() {
    input! {n: usize, m: usize, mut x: [usize; n]}

    if m == n {
        println!("0");
        return;
    }

    x.sort_unstable();

    let mut diff = x.windows(2).map(|v| v[1] - v[0]).collect::<Vec<_>>();
    diff.sort_unstable();
    diff.reverse();

    println!("{}", diff[m - 1..].iter().sum::<usize>())
}
