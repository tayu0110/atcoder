use proconio::input;

fn main() {
    input! {x: usize, y: usize, n: usize}

    let y = std::cmp::min(y, x * 3);

    let res = y * (n / 3);
    println!("{}", res + (n % 3) * x);
}
