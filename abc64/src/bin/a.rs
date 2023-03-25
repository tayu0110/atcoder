use proconio::input;

fn main() {
    input! {_r: usize, g: usize, b: usize}
    if (g * 10 + b) % 4 == 0 {
        println!("YES")
    } else {
        println!("NO")
    }
}
