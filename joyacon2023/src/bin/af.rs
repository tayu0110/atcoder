use proconio::input;

fn main() {
    input! {r: i32, c: i32}

    let d = std::cmp::max((r - 8).abs(), (c - 8).abs());
    if d % 2 == 0 {
        println!("white")
    } else {
        println!("black")
    }
}
