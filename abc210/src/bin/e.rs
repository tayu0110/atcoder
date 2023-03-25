use proconio::input;

fn gcd(x: usize, y: usize) -> usize {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}

fn main() {
    input! {mut n: usize, m: usize, mut p: [(usize, usize); m]}

    p.sort_by_key(|&(a, c)| (c, a));

    let mut res = 0;
    for (a, c) in p {
        let g = gcd(n, a);

        if g == n {
            continue;
        }

        if g == 1 {
            res += (n - 1) * c;
            n = 1;
        } else {
            let k = n / g;
            let l = (k - 1) * g;
            res += l * c;
            n /= k;
        }

        if n == 1 {
            break;
        }
    }

    if n != 1 {
        println!("-1");
    } else {
        println!("{}", res);
    }
}
