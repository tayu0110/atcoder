use proconio::input;

fn main() {
    input! {n: u128, r: u128}
    let (mut den, mut num) = (1, 1);
    for i in 0..r {
        den *= n - i;
        num *= r - i;
    }
    println!("{}", den / num);
}
