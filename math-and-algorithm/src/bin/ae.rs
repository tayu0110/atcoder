use proconio::*;

fn main() {
    input! {p: [(f64, f64); 3]}

    let ba = (p[0].0 - p[1].0, p[0].1 - p[1].1);
    let bc = (p[2].0 - p[1].0, p[2].1 - p[1].1);
    let ca = (p[0].0 - p[2].0, p[0].1 - p[2].1);
    let cb = (p[1].0 - p[2].0, p[1].1 - p[2].1);
    let res = if ba.0 * bc.0 + ba.1 * bc.1 < 0.0 {
        (ba.0 * ba.0 + ba.1 * ba.1).sqrt()
    } else if ca.0 * cb.0 + ca.1 * cb.1 < 0.0 {
        (ca.0 * ca.0 + ca.1 * ca.1).sqrt()
    } else {
        (ba.0 * bc.1 - ba.1 * bc.0).abs() / (bc.0 * bc.0 + bc.1 * bc.1).sqrt()
    };
    println!("{res}")
}
