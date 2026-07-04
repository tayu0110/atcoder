use proconio::*;

fn main() {
    input! {n: usize, k: usize}

    let mut res = 0;
    for i in 0..5 {
        let mut st = i + 9;
        let mut c = 0;
        while st * 9 % 11 != k {
            st += 7;
            c += 1;
        }
        if n >= c + 3 {
            res += (n - c - 3) / 11 + 1;
        }
    }
    println!("{res}")
}
