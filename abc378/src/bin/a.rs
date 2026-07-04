use proconio::*;

fn main() {
    input! {mut a: [usize; 4]}

    a.sort();
    if a[0] == a[1] && a[2] == a[3] {
        println!("2")
    } else if a.windows(2).filter(|c| c[0] == c[1]).count() >= 1 {
        println!("1")
    } else {
        println!("0")
    }
}
