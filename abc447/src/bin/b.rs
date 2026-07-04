use proconio::*;

fn main() {
    input! {s: marker::Bytes}
    let mut count = [0; 128];
    for &b in &s {
        count[b as usize] += 1;
    }
    let &max = count.iter().max().unwrap();
    println!(
        "{}",
        s.into_iter()
            .filter(|&b| count[b as usize] != max)
            .map(|b| b as char)
            .collect::<String>()
    )
}
