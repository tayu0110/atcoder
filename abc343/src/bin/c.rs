use proconio::*;

fn main() {
    input! {n: usize}

    let mut res = 1;
    for i in 1..=1000000usize {
        let j = i * i * i;

        if j > n {
            break;
        }

        let s = j.to_string();
        let t = s.clone();
        let s = s.chars().rev().collect::<String>();

        if s == t {
            res = j;
        }
    }

    println!("{res}")
}
