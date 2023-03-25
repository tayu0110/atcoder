use proconio::input;

fn gcd(x: usize, y: usize) -> usize {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}

fn main() {
    input! {n: usize, a: [usize; n]}

    let g = a.iter().fold(0, |s, v| gcd(s, *v));
    let mut res = 0;
    for a in a {
        let mut now = a / g;
        while now % 2 == 0 {
            now /= 2;
            res += 1;
        }
        while now % 3 == 0 {
            now /= 3;
            res += 1;
        }

        if now != 1 {
            println!("-1");
            return;
        }
    }

    println!("{}", res);
}
