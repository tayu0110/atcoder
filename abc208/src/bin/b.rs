use proconio::*;

fn main() {
    input! {mut p: usize}

    let mut res = 0;
    let mut m = (1..=10).product::<usize>();
    for rem in (1..=10).rev() {
        res += p / m;
        p %= m;
        m /= rem;
    }

    println!("{}", res)
}
