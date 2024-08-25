use proconio::input;

fn solve(dig: usize, a: &mut [usize]) -> usize {
    if a.is_empty() {
        return 0;
    }
    if dig == 0 {
        let d = a[0] & 1;
        return if a.iter().all(|v| (*v & 1) == d) {
            0
        } else {
            1
        };
    }

    let mask = (1usize << (dig + 1)) - 1;
    a.iter_mut().for_each(|v| *v &= mask);

    a.sort();
    let mut res = None;

    if a[0] >= 1 << dig {
        res = Some(solve(dig - 1, a));
    } else {
        for i in 0..a.len() {
            if a[i] >= 1 << dig {
                let f0 = solve(dig - 1, &mut a[0..i]);
                let f1 = solve(dig - 1, &mut a[i..]);

                res = Some(std::cmp::min(
                    std::cmp::max(f0, f1 | 1 << dig),
                    std::cmp::max(f1, f0 | 1 << dig),
                ));
                break;
            }
        }

        if res.is_none() {
            res = Some(solve(dig - 1, a));
        }
    }

    res.unwrap()
}

fn main() {
    input! {n: usize, mut a: [usize; n]}

    println!("{}", solve(30, &mut a));
}
