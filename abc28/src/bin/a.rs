use proconio::*;

fn main() {
    input! {n: usize}

    if n <= 59 {
        println!("Bad")
    } else if n <= 89 {
        println!("Good")
    } else if n < 100 {
        println!("Great")
    } else {
        println!("Perfect")
    }
}
