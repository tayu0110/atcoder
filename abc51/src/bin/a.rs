use proconio::*;

fn main() {
    input! {s: String}

    let v = s.split(',').collect::<Vec<_>>();
    println!("{} {} {}", v[0], v[1], v[2])
}
