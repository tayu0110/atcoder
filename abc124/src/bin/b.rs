use proconio::*;

fn main() {
    input! {h: [usize]}

    let mut max = 0;
    let mut res = 0;
    for h in h {
        if h >= max {
            res += 1;
            max = h;
        }
    }

    println!("{}", res)
}
