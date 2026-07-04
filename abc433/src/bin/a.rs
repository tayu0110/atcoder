use proconio::*;

fn main() {
    input! {mut x: usize, mut y: usize, z: usize}
    for _ in 0..10000 {
        if x == y * z {
            println!("Yes");
            return;
        }
        x += 1;
        y += 1;
    }
    println!("No")
}
