use proconio::*;

fn main() {
    input! {n: usize, k: usize}

    if k < 2 * (n - 1) {
        println!("No");
        return;
    }

    let k = k - 2 * (n - 1);

    if k % 2 == 0 {
        println!("Yes")
    } else {
        println!("No")
    }
}
