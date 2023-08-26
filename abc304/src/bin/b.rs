use proconio::*;

fn main() {
    input! {s: marker::Chars}
    if s.len() < 3 {
        println!("{}", s.into_iter().collect::<String>());
        return;
    }

    let mut ns = s[..3].into_iter().copied().collect::<Vec<char>>();
    ns.resize(s.len(), '0');
    println!("{}", ns.into_iter().collect::<String>())
}
