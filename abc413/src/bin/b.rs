use proconio::*;

fn main() {
    input! {n: usize, s: [String; n]}

    let mut t = vec![];
    for i in 0..n {
        for j in 0..n {
            if i != j {
                t.push(format!("{}{}", s[i], s[j]));
            }
        }
    }
    t.sort_unstable();
    t.dedup();
    println!("{}", t.len())
}
