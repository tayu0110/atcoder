use proconio::*;

fn main() {
    input! {x: [usize; 3], s: usize}
    if x.iter().max().unwrap() >= &s {
        println!("Yes")
    } else {
        println!("No")
    }
}
