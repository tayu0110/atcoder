use proconio::*;

fn main() {
    input! {n: usize, a: [i32; n]}
    if a.iter().max().unwrap() > &0 {
        println!("Yes")
    } else {
        println!("No")
    }
}
