use proconio::*;

fn main() {
    input! {x: usize, y: usize}

    for t in 0..=x {
        let b = x - t;

        if t * 4 + 2 * b == y {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
