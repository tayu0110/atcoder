use proconio::*;

fn main() {
    input! {a: usize, b: usize}

    let mut now = 1;
    let mut res = 0;
    while now < b {
        res += 1;
        now = now - 1 + a;
    }

    println!("{}", res)
}
