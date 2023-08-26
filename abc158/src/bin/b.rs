use proconio::*;

fn main() {
    input! {n: usize, a: usize, b: usize}

    let res = a * (n / (a + b));
    let rem = n % (a + b);
    println!("{}", res + if rem >= a { a } else { rem })
}
