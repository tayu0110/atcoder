use proconio::*;

fn main() {
    input! {a: usize, b: usize, s: String}
    if a <= s.len() && s.len() <= b {
        println!("YES")
    } else {
        println!("NO")
    }
}
