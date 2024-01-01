use proconio::*;

fn main() {
    input! {n: usize, m: usize, s: [String; n], t: [String; m]}

    let mut ti = 0;
    for s in s {
        if s == t[ti] {
            println!("Yes");
            ti += 1;
        } else {
            println!("No")
        }
    }
}
