fn main() {
    println!(
        "{}",
        (1..=100)
            .filter(|s| s % 3 != 0 && s % 5 != 0)
            .sum::<usize>()
    )
}
