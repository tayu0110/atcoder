use proconio::*;

fn main() {
    input! {n: usize, k: usize, mut a: [(usize, String); n]}
    a.insert(0, (0, "kogakubu10gokan".to_owned()));
    let pos = a.partition_point(|a| a.0 <= k);
    println!("{}", a[pos - 1].1);
}
