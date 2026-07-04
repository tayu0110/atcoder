use proconio::*;

fn main() {
    input! {n: usize, a: [i32; n]}

    let mut res = 0usize;
    let mut memo = vec![0; 400010];
    for (i, a) in a.into_iter().enumerate() {
        if i as i32 + 1 - a >= 0 {
            res += memo[i + 1 - a as usize];
        }
        memo[i + 1 + a as usize] += 1;
    }

    println!("{res}")
}
