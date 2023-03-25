use proconio::input;

fn main() {
    input! {x: i64, k: i64, d: i64}

    let nx = x.abs();

    if k <= nx / d {
        println!("{}", nx - d * k);
        return;
    }

    let k = k - nx / d;
    let nx = nx % d;
    if k % 2 == 0 {
        println!("{}", nx);
    } else {
        println!("{}", d - nx);
    }
}
