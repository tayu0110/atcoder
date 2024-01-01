use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}

    let g = a.into_iter().fold(0, std::ops::BitXor::bitxor);
    if g == 0 {
        println!("Second")
    } else {
        println!("First")
    }
}
