use proconio::*;

fn main() {
    input! {m: usize, d: usize}
    if [(1, 7), (3, 3), (5, 5), (7, 7), (9, 9)].contains(&(m, d)) {
        println!("Yes")
    } else {
        println!("No")
    }
}
