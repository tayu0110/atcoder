use proconio::*;

fn main() {
    input! {n: usize, e: [(i32, i32); n]}

    let mut memo = [i32::MIN; 100];
    memo[0] = 0;
    let mut new = memo.clone();
    for (p, u) in e {
        for i in 0..100 {
            if memo[i] == i32::MIN {
                continue;
            }

            let v = memo[i] + u - p + ((p as usize + i) / 100 * 20) as i32;
            new[(p as usize + i) % 100] = new[(p as usize + i) % 100].max(v);
        }

        memo = new;
    }

    println!("{}", memo.into_iter().max().unwrap());
}
