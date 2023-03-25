use itertools::Itertools;
use proconio::input;

fn rec(now: i64, res: &mut Vec<i64>) {
    if now == 1 {
        res.push(1);
        return;
    } else if now == 2 {
        res.push(-1);
        return;
    }

    let mut n = now - if now % 3 == 2 { -1 } else { now % 3 };
    let mut k = 1;
    while n % 3 == 0 {
        k *= 3;
        n /= 3;
    }

    rec(n, res);

    res.iter_mut().for_each(|s| *s *= k);
    let n = now - if now % 3 == 2 { -1 } else { now % 3 } - res.iter().sum::<i64>();
    if n != 0 {
        res.push(n);
    }
    if now % 3 == 1 {
        res.push(1);
    } else if now % 3 == 2 {
        res.push(-1);
    }
}

fn main() {
    input! {n: i64}

    if n == 2 {
        println!("2");
        println!("3 -1");
        return;
    }

    let mut res = vec![];
    rec(n, &mut res);

    assert_eq!(res.iter().sum::<i64>(), n);

    println!("{}", res.len());
    println!("{}", res.iter().join(" "))
}
