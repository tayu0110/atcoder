use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n], x: usize}
    if a.contains(&x) {
        println!("Yes")
    } else {
        println!("No")
    }
}
