use proconio::*;

fn main() {
    input! {n: usize, p: [(char, usize); n]}

    if p.iter().map(|p| p.1).fold(0, |s, v| v.saturating_add(s)) > 100 {
        println!("Too Long");
        return;
    }

    let mut buf = String::new();
    for (c, l) in p {
        buf.push_str(&format!("{c}").repeat(l));
    }
    println!("{buf}")
}
