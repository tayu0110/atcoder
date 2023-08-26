use proconio::*;

fn main() {
    input! {n: usize, p: [(char, i64); n]}
    let mut res = 0;
    for &(c, a) in &p {
        if c == '+' {
            res += a;
        }
    }
    for &(c, a) in &p {
        if c == '*' && a > 0 {
            res *= a;
        }
    }

    println!("{}", res)
}
