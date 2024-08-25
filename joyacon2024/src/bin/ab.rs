use proconio::*;

fn main() {
    input! {s: String}

    let a = s.find('A').unwrap();
    let z = s.rfind('Z').unwrap();

    println!("{}", z + 1 - a)
}
