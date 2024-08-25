use proconio::*;

fn main() {
    input! {n: usize, m: usize, mut t: [i32; n]}
    t.resize(3000, 0);

    let mut now = 0;
    for i in 1..=m {
        input! {s: i32}

        now += s;
        now += t[now as usize];
        if now >= n as i32 - 1 {
            println!("{i}");
            return;
        }
    }
}
