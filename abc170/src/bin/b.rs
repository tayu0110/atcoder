use proconio::*;

fn main() {
    input! {x: usize, y: usize}
    for i in 0..=x {
        if i * 4 > y {
            continue;
        }

        let rem = y - i * 4;
        if rem % 2 != 0 {
            continue;
        }

        if i + rem / 2 == x {
            println!("Yes");
            return;
        }
    }

    println!("No")
}
