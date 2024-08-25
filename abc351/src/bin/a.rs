use proconio::*;

fn main() {
    input! {a: [i32; 9], b: [i32; 8]}

    let res = a
        .iter()
        .sum::<i32>()
        .saturating_sub(b.into_iter().sum::<i32>());
    println!("{}", 0.max(res + 1))
}
