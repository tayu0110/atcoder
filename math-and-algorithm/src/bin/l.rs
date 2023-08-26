use proconio::input;

fn main() {
    input! {n: usize}
    if (2..n).take_while(|&j| j * j <= n).all(|v| n % v != 0) {
        println!("Yes")
    } else {
        println!("No")
    }
}
