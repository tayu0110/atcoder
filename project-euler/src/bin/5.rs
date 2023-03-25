fn gcd(x: usize, y: usize) -> usize {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}

fn lcm(x: usize, y: usize) -> usize {
    x / gcd(x, y) * y
}

fn main() {
    let mut res = 1;
    for i in 1..=20 {
        res = lcm(res, i);
    }

    println!("{}", res);
}