use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}

    if a.windows(3).any(|v| v[0] == v[1] && v[1] == v[2]) {
        println!("Yes")
    } else {
        println!("No")
    }
}
