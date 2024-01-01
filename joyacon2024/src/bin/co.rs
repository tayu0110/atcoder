use proconio::*;

fn main() {
    input! {m: usize, d: usize}

    let mut res = 0;
    for i in 1..=m {
        for j in 10..=d {
            let d1 = j / 10;
            let d2 = j % 10;
            if d1 >= 2 && d2 >= 2 && d1 * d2 == i {
                res += 1;
            }
        }
    }

    println!("{res}")
}
