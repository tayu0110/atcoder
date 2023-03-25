use proconio::input;

fn main() {
    input! {a: f64, b: f64, c: f64}

    let f = |x: f64| a * x * x * x * x * x + b * x + c;
    let (mut l, mut r) = (0.9, 2.1);
    while r - l > 1e-10 {
        let x = (r + l) / 2.0;
        let res = f(x);

        if res < 0.0 {
            l = x;
        } else {
            r = x;
        }
    }

    println!("{:.10}", r);
}
