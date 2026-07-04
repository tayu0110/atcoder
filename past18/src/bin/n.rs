use proconio::*;

fn main() {
    input! {tx: f64, ty: f64, vt: f64, ax: f64, ay: f64, va: f64}

    if vt > va {
        println!("inf");
        return;
    }

    let a = vt * vt - va * va;
    let b = va * va * ty - vt * vt * ay;
    let c = vt * vt * (ax * ax + ay * ay) - va * va * (tx * tx + ty * ty);
    if vt == va {
        if b == 0. {
            if c < 0. {
                println!("0")
            } else {
                println!("inf")
            }
        } else {
            if c / b > 0. {
                println!("0")
            } else {
                println!("inf")
            }
        }
    } else {
        let f = b * b - a * c;
        if f < 0. {
            println!("0")
        } else {
            println!("{}", 2. * f.sqrt() / -a);
        }
    }
}
