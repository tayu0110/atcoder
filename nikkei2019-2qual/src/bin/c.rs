use std::{cmp::Reverse, collections::BTreeSet};

use proconio::input;

fn main() {
    input! {n: usize, a: [usize; n], b: [usize; n]}

    let mut c = b.into_iter().zip(a).collect::<Vec<_>>();
    c.sort();
    let (b, mut a) = c.into_iter().unzip::<usize, usize, Vec<_>, Vec<_>>();
    let mut set = BTreeSet::new();
    for i in 0..n {
        set.insert((a[i], Reverse(i)));
    }

    let mut cnt = 0;
    for i in 0..n {
        let mut bad = true;
        // eprintln!("a: {:?}", a);
        // eprintln!("b: {:?}", b);
        // eprintln!("set: {:?}", set);
        while let Some(&(na, Reverse(j))) = set.range(..=(b[i], Reverse(0))).next_back() {
            if j == i {
                bad = false;
                break;
            }

            if j < i {
                set.remove(&(na, Reverse(j)));
                continue;
            }

            set.remove(&(a[i], Reverse(i)));
            set.remove(&(na, Reverse(j)));
            a.swap(i, j);
            set.insert((a[j], Reverse(j)));
            cnt += 1;
            bad = false;
            break;
        }

        if bad {
            println!("No");
            return;
        }
    }

    eprintln!("cnt: {}", cnt);

    if cnt <= n - 2 {
        println!("Yes")
    } else {
        println!("No")
    }
}
