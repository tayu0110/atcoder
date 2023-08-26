use proconio::*;

fn main() {
    input! {mut a: usize, mut b: usize}

    let mut res = 0;
    while b > 0 {
        if a % b == 0 {
            res += a / b - 1;
            break;
        }

        res += a / b;
        a %= b;
        std::mem::swap(&mut a, &mut b);
    }

    println!("{}", res)
}
