use proconio::*;

fn main() {
    input! {n: usize, k: u32, a: [u32; n]}

    let mut res = 0;
    for i in 0u16..1 << n {
        if i.count_ones() == k {
            for j in 0..n {
                if i & (1 << j) != 0 {
                    res += a[j];
                }
            }
        }
    }

    println!("{res}")
}
