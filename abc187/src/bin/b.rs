use proconio::*;

fn main() {
    input! {n: usize, p: [(i32, i32); n]}

    let mut res = 0;
    for (i, &(x, y)) in p.iter().enumerate() {
        for &(nx, ny) in p.iter().skip(i + 1) {
            if (y - ny).abs() <= (x - nx).abs() {
                res += 1;
            }
        }
    }
    println!("{}", res)
}
