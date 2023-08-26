use proconio::*;

fn main() {
    input! {s: [usize; 8]}

    if s.windows(2).all(|v| v[0] <= v[1]) && s.iter().all(|&v| 100 <= v && v <= 675 && v % 25 == 0)
    {
        println!("Yes")
    } else {
        println!("No")
    }
}
