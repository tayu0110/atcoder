use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut cnt = [0; 101];
    for a in a {
        cnt[a] += 1;
    }

    if cnt.iter().any(|&a| (1..=10).contains(&a)) {
        println!("Yes")
    } else {
        println!("No")
    }
}
