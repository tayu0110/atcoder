use proconio::*;

fn main() {
    input! {mut h: usize}

    let mut cnt = 1;
    let mut res = 0usize;
    while h > 0 {
        res += cnt;
        h /= 2;
        cnt *= 2;
    }

    println!("{}", res);
}
