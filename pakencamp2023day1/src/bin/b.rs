use proconio::*;

fn main() {
    input! {_: usize, mut a: [usize; 2], mut b: [usize; 2]}
    a.sort_unstable();

    if a[0] == b[0] || a[0] == b[1] || a[1] == b[0] || a[1] == b[1] {
        println!("3")
    } else if (a[0] + 1..a[1]).contains(&b[0]) != (a[0] + 1..a[1]).contains(&b[1]) {
        println!("4")
    } else {
        println!("3")
    }
}
