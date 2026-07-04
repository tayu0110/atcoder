use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, s: marker::Bytes}

    let mut res = vec![0; 2 * n];
    for c in b'1'..=b'9' {
        let mut cum = vec![0; n];
        for i in s
            .iter()
            .enumerate()
            .filter_map(|(i, ch)| (*ch == c).then_some(i))
        {
            cum[n - 1 - i] += i + 1;
        }
        for i in (1..n).rev() {
            cum[i - 1] += cum[i];
        }
        for i in 0..n {
            let mut now = cum[i] * (c - b'0') as usize;
            let mut carry = 0;
            let mut index = i;
            while now > 0 || carry > 0 {
                if index == res.len() {
                    res.push(0);
                }
                let next = res[index] + carry + now % 10;
                res[index] = next % 10;
                carry = next / 10;
                now /= 10;
                index += 1;
            }
        }
    }

    while res.len() > 1 && *res.last().unwrap() == 0 {
        res.pop();
    }
    res.reverse();
    println!("{}", res.iter().join(""));
}
