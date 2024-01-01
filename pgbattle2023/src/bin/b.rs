use proconio::*;

fn main() {
    input! {s: marker::Chars}

    let mut res = 0.0;
    for v in s.windows(3) {
        if v[0] != 'A' && v[0] != '?' {
            continue;
        }
        if v[1] != 'B' && v[1] != '?' {
            continue;
        }
        if v[2] != 'C' && v[2] != '?' {
            continue;
        }

        let cnt = v.iter().filter(|&&c| c == '?').count();
        res += 1.0 / 3.0f64.powi(cnt as i32);
    }

    println!("{res}");
}
