use proconio::*;

fn main() {
    input! {d: usize, f: usize}
    let mut ret = ((d + 6) / 7 * 7 + f - d) % 7;
    if ret == 0 {
        ret = 7;
    }
    println!("{}", ret)
}
