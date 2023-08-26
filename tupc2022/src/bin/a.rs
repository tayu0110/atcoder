use proconio::*;

fn main() {
    input! {n: usize, k: usize, mut p: [usize; n]}

    let mut q = vec![];
    for i in 0..n {
        if p[i] == 1 || p[i] != i + 1 {
            q.push(p[i]);
        }
    }

    if 1 + q.iter().max().unwrap() <= k {
        println!("Yes")
    } else {
        println!("No")
    }
}
