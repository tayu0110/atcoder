use proconio::*;

fn main() {
    input! {l: usize, r: usize}

    if l + r != 1 {
        println!("Invalid")
    } else if l == 1 {
        println!("Yes")
    } else {
        println!("No")
    }
}
