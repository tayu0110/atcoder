use proconio::*;

fn main() {
    input! {mut a: [u8; 3]}

    a.sort();
    if a == vec![5, 5, 7] {
        println!("YES")
    } else {
        println!("NO")
    }
}
