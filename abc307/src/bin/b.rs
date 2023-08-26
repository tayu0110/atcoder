use proconio::*;

fn main() {
    input! {n: usize, s: [marker::Chars; n]}

    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            let s = [s[i].clone(), s[j].clone()].concat();
            let len = s.len();
            let mut bad = false;
            for k in 0..len {
                let l = len - 1 - k;
                if s[l] != s[k] {
                    bad = true;
                    break;
                }
            }

            if !bad {
                println!("Yes");
                return;
            }
        }
    }

    println!("No")
}
