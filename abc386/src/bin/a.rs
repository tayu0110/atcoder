use proconio::*;

fn main() {
    input! {a: [usize; 4]}

    let mut cnt = [0; 15];
    for a in a {
        cnt[a] += 1;
    }

    let &min = cnt.iter().filter(|&&a| a > 0).min().unwrap();
    let &max = cnt.iter().max().unwrap();
    if min == 2 && max == 2 {
        println!("Yes")
    } else if min == 1 && max == 3 {
        println!("Yes")
    } else {
        println!("No")
    }
}
