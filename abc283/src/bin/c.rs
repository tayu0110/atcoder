use proconio::input;

fn main() {
    input! {s: String}
    let s = s.replace("00", "1");
    println!("{}", s.len());
}
