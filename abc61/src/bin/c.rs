use proconio::*;

fn main() {
    input! {n: usize, k: usize, e: [(usize, usize); n]}

    let mut v = vec![0; 100001];
    for (a, b) in e {
        v[a] += b;
    }

    for i in 0..100000 {
        v[i + 1] += v[i];
        if v[i + 1] >= k {
            println!("{}", i + 1);
            return;
        }
    }
}
