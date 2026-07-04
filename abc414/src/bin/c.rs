use proconio::*;

fn main() {
    input! {a: usize, n: usize}

    let len = n.to_string().len();
    let mut buf = [0; 80];
    let mut rbuf = [0; 80];
    let mut res = 0;

    let mut solve = |mut t: usize| {
        if t <= n {
            let keep = t;
            let mut i = 0;
            while t > 0 {
                buf[i] = t % a;
                t /= a;
                i += 1;
            }
            for j in 0..i {
                rbuf[j] = buf[i - 1 - j];
            }
            if buf[..i] == rbuf[..i] {
                res += keep;
            }
        }
    };
    for i in 1..=9.min(n) {
        solve(i);
    }
    for i in 1.. {
        let si = i.to_string();
        if si.len() * 2 > len {
            break;
        }

        let rsi = si.chars().rev().collect::<String>();
        let t = format!("{si}{rsi}").parse::<usize>().unwrap();
        solve(t);

        if si.len() * 2 + 1 > len {
            continue;
        }

        for c in '0'..='9' {
            let t = format!("{si}{c}{rsi}").parse::<usize>().unwrap();
            solve(t);
        }
    }

    println!("{res}")
}
