use proconio::*;

fn main() {
    input! {x: u128, y: u128}

    let mut res = 1;
    let mut now = x;
    while now * 2 <= y {
        now *= 2;
        res += 1;
    }

    println!("{}", res)
}
