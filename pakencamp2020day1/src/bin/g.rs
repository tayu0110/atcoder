use proconio::*;

fn main() {
    input! {n: usize, m: usize, p: [(usize, usize, usize); m]}

    let mut res = -1;
    'b: for i in 0usize..1 << n {
        for &(l, r, x) in &p {
            let mask = (i >> (l - 1)) & ((1 << (r - l + 1)) - 1);
            if mask.count_ones() as usize != x {
                continue 'b;
            }
        }

        res = res.max(i.count_ones() as i32);
    }

    println!("{res}")
}
