#[allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
#[allow(unused_imports)]
use itertools::Itertools;

#[fastout]
fn main() {
    input! {n: usize, s: Chars, t: Chars}

    let mut u = vec!['0'; n];
    let mut ham = 0;
    let mut st = std::collections::BinaryHeap::new();
    let mut tt = std::collections::BinaryHeap::new();
    let mut decieded = std::collections::HashSet::new();
    for i in 0..n {
        if s[i] == t[i] {
            u[i] = '0';
            decieded.insert(i);
        } else {
            ham += 1;
            if s[i] == '1' {
                st.push(std::cmp::Reverse(i));
            } else {
                tt.push(std::cmp::Reverse(i));
            }
        }
    }

    if ham % 2 == 1 {
        println!("-1");
        return;
    }

    while !st.is_empty() && !tt.is_empty() {
        let (mut si, mut ti) = (None, None);
        while let Some(std::cmp::Reverse(i)) = st.pop() {
            if decieded.contains(&i) {
                continue;
            }
            si = Some(i);
            break;
        }
        while let Some(std::cmp::Reverse(i)) = tt.pop() {
            if decieded.contains(&i) {
                continue;
            }
            ti = Some(i);
            break;
        }

        if si.is_none() && ti.is_none() {
            break;
        } else if si.is_none() {
            tt.push(std::cmp::Reverse(ti.unwrap()));
            break;
        } else if ti.is_none() {
            st.push(std::cmp::Reverse(si.unwrap()));
            break;
        }

        let (si, ti) = (si.unwrap(), ti.unwrap());
        u[si] = '0';
        u[ti] = '0';
        decieded.insert(si);
        decieded.insert(ti);
    }

    let mut dq = if !st.is_empty() {
        let mut v = st.into_iter().map(|std::cmp::Reverse(i)| i).collect::<Vec<_>>();
        v.sort();
        v.into_iter().collect::<std::collections::VecDeque<_>>()
    } else {
        let mut v = tt.into_iter().map(|std::cmp::Reverse(i)| i).collect::<Vec<_>>();
        v.sort();
        v.into_iter().collect::<std::collections::VecDeque<_>>()
    };
    while !dq.is_empty() {
        let (mut f, mut b) = (None, None);
        while let Some(i) = dq.pop_front() {
            if decieded.contains(&i) {
                continue;
            }
            f = Some(i);
            break;
        }
        while let Some(i) = dq.pop_back() {
            if decieded.contains(&i) {
                continue;
            }
            b = Some(i);
            break;
        }
        if f.is_none() && b.is_none() {
            break;
        }
        let (f, b) = (f.unwrap(), b.unwrap());
        u[f] = '0';
        u[b] = '1';
        decieded.insert(f);
        decieded.insert(b);
    }
    println!("{}", u.iter().join(""));
}
