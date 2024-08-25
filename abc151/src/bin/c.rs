use proconio::*;

fn main() {
    input! {n: usize, m: usize, p: [(usize, String); m]}

    let mut res = vec![false; n];
    let mut penalty = vec![0; n];
    for (p, s) in p {
        if s == "AC" {
            res[p - 1] = true;
        } else if !res[p - 1] {
            penalty[p - 1] += 1;
        }
    }

    println!(
        "{} {}",
        res.iter().filter(|&&f| f).count(),
        (0..n)
            .filter(|&i| res[i])
            .map(|i| penalty[i])
            .sum::<usize>()
    )
}
