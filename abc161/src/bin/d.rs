use proconio::*;

fn main() {
    input! {k: usize}
    let mut res = vec![0usize; 11];
    let mut dig = 10;
    for _ in 0..k {
        let mut found = false;
        for i in (dig..11).rev() {
            if res[i - 1] >= res[i] && res[i] != 9 {
                res[i] += 1;
                for j in i + 1..11 {
                    res[j] = res[j - 1].saturating_sub(1);
                }
                found = true;
                break;
            }
        }

        if !found {
            if res[dig] < 9 {
                res[dig] += 1;
                for i in dig + 1..11 {
                    res[i] = res[i - 1].saturating_sub(1);
                }
            } else {
                dig -= 1;
                res[dig] = 1;
                for i in dig + 1..11 {
                    res[i] = 0;
                }
            }
        }
    }
    println!("{}", res.iter().fold(0, |s, v| s * 10 + v))
}
