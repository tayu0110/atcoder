use proconio::input;

fn main() {
    input! {n: usize, mut a: [usize; n]}
    a.sort();
    a.dedup();
    let mut a = a.into_iter().collect::<std::collections::VecDeque<_>>();

    let mut prev = 0;
    let mut dup = n - a.len();
    while let Some(now) = a.pop_front() {
        if prev + 1 == now {
            prev = now;
        } else {
            a.push_front(now);
            if dup >= 2 {
                dup -= 2;
                prev += 1;
            } else if dup == 1 {
                if a.len() < 1 {
                    break;
                }

                a.pop_back().unwrap();
                dup -= 1;
                prev += 1;
            } else {
                if a.len() < 2 {
                    break;
                }

                a.pop_back().unwrap();
                a.pop_back().unwrap();
                prev += 1;
            }
        }
    }

    prev += dup / 2;

    println!("{}", prev);
}
