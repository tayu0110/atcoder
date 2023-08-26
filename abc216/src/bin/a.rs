use proconio::*;

fn main() {
    input! {x: String}
    let s = x.split(".").collect::<Vec<_>>();
    let y: usize = s[1].parse().unwrap();

    if y <= 2 {
        println!("{}-", s[0]);
    } else if y <= 6 {
        println!("{}", s[0]);
    } else {
        println!("{}+", s[0]);
    }
}
