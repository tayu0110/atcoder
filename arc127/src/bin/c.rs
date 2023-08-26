use proconio::*;

fn dec(mut x: Vec<u8>) -> Option<Vec<u8>> {
    let len = x.len();
    for i in (0..len).rev() {
        if x[i] == b'1' {
            x[i] -= 1;
            for j in i + 1..len {
                x[j] += 1;
            }
            break;
        }
        if i == 0 {
            return None;
        }
    }
    Some(x)
}

fn main() {
    input! {n: usize, x: marker::Bytes}
    let x = {
        let mut buf = vec![b'0'; n - x.len()];
        buf.extend(x.iter());
        buf
    };

    let mut res = "1".to_string();
    if let Some(mut x) = dec(x) {
        let len = x.len();
        for i in 0..len {
            if x[i] == b'1' {
                x[i] -= 1;
                res.push('1');
            } else if let Some(nx) = dec(x) {
                res.push('0');
                x = nx;
            } else {
                break;
            }
        }
    }

    println!("{}", res);
}
