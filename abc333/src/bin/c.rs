use proconio::*;

fn main() {
    input! {n: usize}

    let mut cnt = 0;
    for i in 1.. {
        for j in 1..=i {
            for k in 1..=j {
                cnt += 1;
                if cnt == n {
                    let a = (0..i).fold(0usize, |s, _| s * 10 + 1);
                    let b = (0..j).fold(0, |s, _| s * 10 + 1);
                    let c = (0..k).fold(0, |s, _| s * 10 + 1);
                    println!("{}", a + b + c);
                    return;
                }
            }
        }
    }
}
