use proconio::*;

fn main() {
    input! {n: usize}

    let mut now = n;
    let mut res = 0;
    for i in (2..=n).take_while(|&i| i * i <= n) {
        let mut cnt = 0;
        while now % i == 0 {
            cnt += 1;
            now /= i;
        }

        if cnt == 0 {
            continue;
        }

        for i in 1.. {
            if i <= cnt {
                cnt -= i;
                res += 1;
            } else {
                break;
            }
        }

        if now == 1 {
            break;
        }
    }

    if now != 1 {
        res += 1;
    }

    println!("{}", res)
}
