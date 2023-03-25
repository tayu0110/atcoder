use proconio::input;

fn main() {
    input! {n: usize, h: [usize; n]}
    let max = *h.iter().max().unwrap();
    println!("{}", h.into_iter().position(|v| v == max).unwrap() + 1);
}
