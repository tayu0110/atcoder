use proconio::*;
use std::io::Write;

fn main() {
    input_interactive!(n: usize);

    let mut s = "01".repeat(n);
    let mut t = String::new();
    while t.len() < n {
        let c = s.pop().unwrap();
        println!("? {}{}", s, t);
        std::io::stdout().flush().ok();
        input_interactive!(u: String);
        if u == "No" {
            t = format!("{c}{t}");
        }
        if s.len() + t.len() == n {
            t = format!("{s}{t}");
        }
    }
    println!("! {t}");
    std::io::stdout().flush().ok();
    return;
}
