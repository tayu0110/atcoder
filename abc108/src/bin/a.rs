use proconio::*;

fn main() {
    input! {k: usize}

    let mut res = 0;
    for i in 1..=k {
        for j in i + 1..=k {
            if i % 2 != j % 2 {
                res += 1;
            }
        }
    }

    println!("{}", res)
}
