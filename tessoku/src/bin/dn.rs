use proconio::*;

#[fastout]
fn main() {
    input! {mut x: u32, mut y: u32}

    let mut res = vec![];
    while x > 1 || y > 1 {
        res.push((x, y));
        if x > y {
            x -= y;
        } else {
            y -= x;
        }
    }

    println!("{}", res.len());
    for (x, y) in res.into_iter().rev() {
        println!("{} {}", x, y);
    }
}
