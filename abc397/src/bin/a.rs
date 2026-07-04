use proconio::*;

fn main() {
    input! {mut x: String}

    let x = x.chars().filter(|&x| x != '.').collect::<String>();
    let x = x.parse::<usize>().unwrap();
    if x >= 380 {
        println!("1")
    } else if x >= 375 {
        println!("2")
    } else {
        println!("3")
    }
}
