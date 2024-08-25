use proconio::*;

fn count(c: &[char], target: char) -> usize {
    c.iter().filter(|&&c| c == target).count()
}

fn solve(s: &mut Vec<char>, t: &mut Vec<char>) -> bool {
    let sa = count(s, 'A');
    let sb = count(s, 'B');
    let sc = count(s, 'C');

    let ta = count(t, 'A');
    let tb = count(t, 'B');
    let tc = count(t, 'C');

    if sc < tc {
        return false;
    }

    if sa > ta || sb > tb {
        return false;
    }

    let len = s.len();
    let mut rem_c = sc - tc;
    let mut da = ta - sa;
    for i in 0..len {
        if rem_c == 0 {
            break;
        }

        if s[i] == 'C' {
            if da == 0 {
                s[i] = 'B';
            } else {
                s[i] = 'A';
                da -= 1;
            }
            rem_c -= 1;
        }
    }

    let mut last = s.len();
    while let (Some(cs), Some(ct)) = (s.pop(), t.pop()) {
        if cs != ct {
            if cs == 'A' {
                return false;
            }
            if s.is_empty() {
                return false;
            }

            last = last.min(s.len() - 1);
            while s[last] == 'B' {
                last -= 1;
            }

            if s[last] != 'A' {
                return false;
            }

            s[last] = 'B';
        }
    }

    true
}

#[proconio::fastout]
fn main() {
    input! {t: usize}

    let mut res = vec![];
    'B: for _ in 0..t {
        input! {_: usize, x: marker::Chars, y: marker::Chars}

        if x == y {
            res.push(true);
            continue;
        }

        let mut ns = vec![];
        let mut nt = vec![];

        for (x, y) in x.into_iter().zip(y) {
            if y == 'C' && x != 'C' {
                res.push(false);
                continue 'B;
            }

            if y == 'C' {
                if !solve(&mut ns, &mut nt) {
                    res.push(false);
                    continue 'B;
                }
                continue;
            }

            ns.push(x);
            nt.push(y);
        }

        if !ns.is_empty() && !solve(&mut ns, &mut nt) {
            res.push(false);
            continue 'B;
        }

        res.push(true);
    }

    for res in res {
        if res {
            println!("Yes")
        } else {
            println!("No")
        }
    }
}
