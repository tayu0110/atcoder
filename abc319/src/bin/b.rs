use proconio::*;

fn main() {
    input! {n: usize}

    let mut f = vec![];
    for i in 1..=9 {
        if n % i == 0 {
            f.push(i);
        }
    }

    let mut s = String::new();
    for i in 0..=n {
        let mut ok = false;
        for &j in &f {
            let k = n / j;
            if i % k == 0 {
                s.push((j as u8 + b'0') as char);
                ok = true;
                break;
            }
        }

        if !ok {
            s.push('-');
        }
    }

    println!("{s}")
}
