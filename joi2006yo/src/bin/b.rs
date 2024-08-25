use proconio::*;

fn main() {
    input! {map: [(char, char)], s: [char]}

    let mut t = [b'\0'; 128];
    for i in 0..128 {
        t[i] = i as u8;
    }
    for (from, to) in map {
        t[from as usize] = to as u8;
    }
    println!(
        "{}",
        s.into_iter()
            .map(|c| t[c as usize] as char)
            .collect::<String>()
    )
}
