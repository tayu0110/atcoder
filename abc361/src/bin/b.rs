use proconio::*;

fn main() {
    input! {a: [usize; 6], b: [usize; 6]}

    if (0..3).any(|i| a[i + 3] <= b[i] || b[i + 3] <= a[i]) {
        println!("No")
    } else {
        println!("Yes")
    }
}
