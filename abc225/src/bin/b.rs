use proconio::*;

fn main() {
    input! {n: usize, p: [(usize, usize); n-1]}
    let mut t = vec![0; n + 1];
    for (u, v) in p {
        t[u] += 1;
        t[v] += 1;
    }

    if t.iter().any(|&v| v == n - 1) {
        println!("Yes")
    } else {
        println!("No")
    }
}
