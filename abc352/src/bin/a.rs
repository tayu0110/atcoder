use proconio::*;

fn main() {
    input! {_: usize, x: usize, y: usize, z: usize}

    if (x.min(y)..=x.max(y)).contains(&z) {
        println!("Yes")
    } else {
        println!("No")
    }
}
