use proconio::*;

fn main() {
    input! {mut n: usize}

    let (mut res, mut val) = (1, 2);
    while n > 0 {
        if n & 1 != 0 {
            res *= val;
            res %= 10;
        }
        val *= val;
        val %= 10;
        n >>= 1;
    }

    println!("{res}")
}
