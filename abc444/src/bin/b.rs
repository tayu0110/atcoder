use proconio::*;

fn main() {
    input! {n: usize, k: usize}
    let mut ret = 0;
    for i in 1..=n {
        if i.to_string().bytes().map(|b| b - b'0').sum::<u8>() as usize == k {
            ret += 1;
        }
    }

    println!("{ret}")
}
