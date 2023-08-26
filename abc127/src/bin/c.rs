use proconio::*;

fn main() {
    input! {n: usize, m: usize, p: [(usize, usize); m]}

    let mut cum = vec![0; n + 2];
    for (l, r) in p {
        cum[l] += 1;
        cum[r + 1] -= 1;
    }

    for i in 0..n + 1 {
        cum[i + 1] += cum[i];
    }

    println!("{}", cum.into_iter().filter(|&c| c == m as i32).count())
}
