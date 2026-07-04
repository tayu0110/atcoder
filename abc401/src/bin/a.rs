use proconio::*;

fn main() {
    input! {s: usize}

    if (200..=299).contains(&s) {
        println!("Success")
    } else {
        println!("Failure")
    }
}
