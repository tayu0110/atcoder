use proconio::*;

fn main() {
    input! {n: usize}

    let mut res = 0;
    for i in 1..=n {
        res += n / i;
        if n % i == 0 {
            res -= 1;
        }
    }

    println!("{}", res)
}
