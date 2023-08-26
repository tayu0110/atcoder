use proconio::*;

fn main() {
    input! {n: usize}
    let s = format!("{:b}", n).chars().collect::<Vec<_>>();
    let len = s.len();
    for i in 0..len {
        let j = len - 1 - i;
        if s[i] != s[j] {
            println!("No");
            return;
        }
    }

    println!("Yes")
}
