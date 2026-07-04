use proconio::*;

fn main() {
    input! {n: usize, m: usize, p: [(usize, usize); m]}

    let mut t = vec![0i32; n + 2];
    for (l, r) in p {
        t[l] += 1;
        t[r + 1] -= 1;
    }

    for i in 0..=n {
        t[i + 1] += t[i];
    }

    println!("{}", t[1..=n].iter().min().unwrap())
}
