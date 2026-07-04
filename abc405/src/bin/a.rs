use proconio::*;

fn main() {
    input! {r: usize, x: usize}
    let f = if x == 1 {
        (1600..=2999).contains(&r)
    } else {
        (1200..=2399).contains(&r)
    };

    if f {
        println!("Yes")
    } else {
        println!("No")
    }
}
