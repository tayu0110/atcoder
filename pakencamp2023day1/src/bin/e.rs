use proconio::*;

fn main() {
    input! {n: usize, m: usize, k: usize, e: [(usize, usize); m]}

    let mut ins = vec![0; n + 1];
    for (a, b) in e {
        ins[a] += k;
        ins[b] += k;
    }

    if ins.into_iter().filter(|i| i % 2 == 1).count() <= 2 {
        println!("Yes")
    } else {
        println!("No")
    }
}
