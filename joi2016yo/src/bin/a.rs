use proconio::*;

fn main() {
    input! {a: [usize; 4], b: [usize; 2]}

    println!(
        "{}",
        a.iter().sum::<usize>() + b.iter().sum::<usize>()
            - a.iter().min().unwrap()
            - b.iter().min().unwrap()
    )
}
