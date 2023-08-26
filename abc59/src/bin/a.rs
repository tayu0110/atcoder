use proconio::*;

fn main() {
    input! {s: [marker::Chars; 3]}

    println!(
        "{}",
        vec![s[0][0], s[1][0], s[2][0]]
            .into_iter()
            .map(|c| c.to_uppercase().next().unwrap())
            .collect::<String>()
    )
}
