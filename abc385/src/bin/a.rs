use proconio::*;

fn main() {
    input! {mut a: [u32; 3]}
    a.sort();
    if a[0] + a[1] == a[2] || (a[0] == a[1] && a[1] == a[2]) {
        println!("Yes")
    } else {
        println!("No")
    }
}
