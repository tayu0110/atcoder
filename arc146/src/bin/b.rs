use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, m: usize, k: usize, a: [usize; n]};

    let mut a = a.into_iter().zip(vec![0; n]).collect_vec();

    let mut res = 0;
    for i in (0..31).rev() {
        let mut tmp = vec![];
        let target = 1usize << i;

        for (now, cnt) in a.iter_mut() {
            if (*now & target) != 0 {
                let nt = *now ^ target;
                tmp.push((nt, *cnt));
            } else if *cnt + target - *now <= m {
                tmp.push((0, *cnt + target - *now));
            }

            *now &= !target;
        }

        if tmp.len() < k {
            continue;
        }

        tmp.sort_by(|l, r| l.1.cmp(&r.1));
        let sum = tmp.iter().take(k).fold(0, |s, (_, cnt)| s + *cnt);

        if sum <= m {
            res += target;
            std::mem::swap(&mut a, &mut tmp);
        }
    }

    println!("{}", res);
}
