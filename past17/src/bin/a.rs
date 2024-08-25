use proconio::*;

fn main() {
    input! {mut h: f64, w: f64}

    h /= 100.;
    let bmi = w / h.powi(2);
    if bmi < 20. {
        println!("A")
    } else if bmi < 25. {
        println!("B")
    } else {
        println!("C")
    }
}
