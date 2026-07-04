use proconio::*;

fn main() {
    input! {x: usize}
    println!(
        "{}",
        "HelloWorld"
            .chars()
            .enumerate()
            .filter_map(|(i, c)| (i != x - 1).then_some(c))
            .collect::<String>()
    )
}
