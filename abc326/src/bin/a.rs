use proconio::*;

fn main() {
    input! {x: usize, y: usize}

    if x > y {
        if x - y <= 3 {
            println!("Yes")
        } else {
            println!("No")
        }
    } else if y - x <= 2 {
        println!("Yes")
    } else {
        println!("No")
    }
}
