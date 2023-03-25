use proconio::input;

fn gcd(x: usize, y: usize) -> usize {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}

fn main() {
    input! {n: usize, mut a: [usize; n]}
    a.sort();
    a.dedup();

    if a.len() == 1 {
        println!("1");
        return;
    }

    let n = a.len();
    let mut b = vec![];
    for i in 1..n {
        b.push(a[i] - a[0]);
    }

    let g = b.iter().fold(0, |s, v| gcd(s, *v));

    if g == 1 {
        println!("2")
    } else {
        println!("1")
    }
}
