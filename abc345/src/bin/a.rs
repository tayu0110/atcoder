use proconio::*;

fn main() {
    input! {s: String}

    if s.starts_with('<') && s.ends_with('>') && s[1..s.len() - 1].chars().all(|c| c == '=') {
        println!("Yes")
    } else {
        println!("No")
    }
}
