use proconio::*;

fn main() {
    input! {l: usize, r: usize}

    let mut res = 0;
    let diff = r - l + 1;
    for i in 1..=diff {
        let start = (l + i - 1) / i * i;
        for a in (start..=r).step_by(i) {
            let b = a ^ i;
            if (a + 1..=r).contains(&b) && b % i == 0 {
                res += 1;
            }
        }
    }

    println!("{res}")
}
