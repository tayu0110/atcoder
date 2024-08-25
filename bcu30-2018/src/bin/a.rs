use proconio::*;

fn main() {
    input! {a: [usize], b: [usize]}
    if a.iter().product::<usize>() == b.iter().product() {
        println!("Yes")
    } else {
        println!("No")
    }
}
