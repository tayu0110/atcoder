use proconio::*;

fn main() {
    input! {n: usize}

    let mut res = 0;
    'base: for i in 1..=n {
        if i.to_string().contains('7') {
            continue;
        }

        let mut now = i;
        while now > 0 {
            if now % 8 == 7 {
                continue 'base;
            }

            now /= 8;
        }

        res += 1;
    }

    println!("{}", res)
}
