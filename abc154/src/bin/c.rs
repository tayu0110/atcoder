use proconio::*;

fn main() {
    input! {n: usize, mut a: [usize; n]}
    a.sort();
    a.dedup();
    if n == a.len() {
        println!("YES")
    } else {
        println!("NO")
    }
}
