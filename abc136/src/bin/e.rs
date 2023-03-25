use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, k: i64, a: [i64; n]};

    let sum = a.iter().sum::<i64>();

    let mut d = (1..=sum)
        .take_while(|v| *v * *v <= sum)
        .filter(|v| sum % *v == 0)
        .fold(vec![], |mut v, s| {
            v.push(s);
            if s * s != sum {
                v.push(sum / s);
            }
            v
        });
    d.sort_by_key(|v| std::cmp::Reverse(*v));

    for res in d {
        let mut a = a.iter().map(|v| *v % res).filter(|v| *v != 0).collect_vec();
        a.sort();
        let mut a = a.into_iter().collect::<std::collections::VecDeque<i64>>();
        
        let mut nk = 0;
        while let Some(mut f) = a.pop_front() {
            while let Some(b) = a.pop_back() {
                if res - b > f {
                    nk += f;
                    a.push_back(b + f);
                    break;
                }

                f -= res - b;
                nk += res - b;
                if f == 0 {
                    break;
                }
            }
        }

        if nk <= k {
            println!("{}", res);
            std::process::exit(0);
        }
    }
}
