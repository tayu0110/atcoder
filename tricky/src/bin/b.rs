use proconio::*;
// use rand::{thread_rng, Rng};

fn main() {
    use std::io::Write;
    let out = std::io::stdout();
    let mut out = std::io::BufWriter::new(out.lock());

    input! {t: usize}
    // let mut rng = thread_rng();

    for _ in 0..t {
        input! {a: i128, b: i128, c: i128}
        // let a = rng.gen::<i32>() as i128;
        // let b = rng.gen::<i32>() as i128;
        // let c = rng.gen::<i32>() as i128;
        // eprintln!("a: {}, b: {}, c: {}", a, b, c);

        if a == 0 && b == 0 && c == 0 {
            writeln!(out, "3").unwrap();
            continue;
        }

        if a == 0 && b == 0 {
            writeln!(out, "0").unwrap();
            continue;
        } else if (a == 0 || b == 0) && c == 0 {
            writeln!(out, "1 0").unwrap();
            continue;
        }

        if a == 0 {
            writeln!(out, "1 {}", -(c as f64) / b as f64).unwrap();
            continue;
        }

        let b2 = b * b;
        let ac = 4 * a * c;
        if b2 < ac {
            writeln!(out, "0").unwrap();
        } else if b2 == ac {
            writeln!(out, "1 {}", -b as f64 / (2 * a) as f64).unwrap()
        } else {
            let dr = ((b2 - ac) as f64).sqrt();
            let mut res = [(-(b as f64) - dr) / (2 * a) as f64,
                (-(b as f64) + dr) / (2 * a) as f64];
            res.sort_by(|l, r| l.partial_cmp(r).unwrap());
            writeln!(out, "2 {} {}", res[0], res[1]).unwrap()
        }
    }
}
