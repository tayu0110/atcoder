use proconio::*;

fn main() {
    input! {x: String, y: String}
    let t = ["Ocelot", "Serval", "Lynx"];
    if t.iter().position(|&t| t == x) >= t.iter().position(|&t| t == y) {
        println!("Yes")
    } else {
        println!("No")
    }
}
