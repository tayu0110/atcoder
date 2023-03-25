use proconio::input;

fn main() {
    input! {mut n: i32}

    if n == 0 {
        println!("0");
        return;
    }

    let mut res = vec![];
    while n != 0 {
        if n % 2 == 0 {
            res.push('0');
            n /= -2;
        } else {
            res.push('1');
            let t = n / -2;
            if n - t * -2 < 0 {
                if n - (t + 1) * -2 >= 0 {
                    n = t + 1;
                } else {
                    n = t - 1;
                }
            } else {
                n = t;
            }
        }
    }

    res.reverse();
    println!("{}", res.into_iter().collect::<String>())
}
