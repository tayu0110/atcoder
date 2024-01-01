use proconio::*;

fn main() {
    input! {n: usize, m: usize, e: [(usize, usize); m]}

    let mut res = vec![0; n];
    for (a, b) in e {
        res[a - 1] += 1;
        res[b - 1] += 1;
    }

    let max = res.iter().max().unwrap();
    println!("{}", res.iter().position(|r| r == max).unwrap() + 1);
}
