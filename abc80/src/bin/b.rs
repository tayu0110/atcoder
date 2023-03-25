use proconio::*;

fn main() {
    input! {n: usize}

    let mut now = n;
    let mut sum = 0;
    while now > 0 {
        sum += now % 10;
        now /= 10;
    }

    if n % sum == 0 {
        println!("Yes")
    } else {
        println!("No")
    }
}
