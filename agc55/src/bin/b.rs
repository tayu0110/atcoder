use proconio::{input, marker::Bytes};

fn convert(s: Vec<u8>) -> Vec<u8> {
    let mut stack = vec![];
    for c in s.into_iter().rev() {
        stack.push(c);

        if stack.len() >= 3 {
            let mut ns = stack[stack.len() - 3..].to_vec();
            ns.reverse();

            if [b"ABC", b"BCA", b"CAB"].iter().any(|&t| t[..] == ns[..]) {
                for _ in 0..3 {
                    stack.pop().unwrap();
                }
            }
        }
    }

    stack.reverse();
    stack
}

fn main() {
    input! {_: usize, s: Bytes, t: Bytes}

    {
        let mut a = std::collections::BTreeMap::new();
        let mut b = std::collections::BTreeMap::new();
        for &c in &s {
            *a.entry(c).or_insert(0) += 1;
        }
        for &c in &t {
            *b.entry(c).or_insert(0) += 1;
        }

        if a != b {
            println!("NO");
            return;
        }
    }

    let s = convert(s);
    let t = convert(t);

    if s == t {
        println!("YES")
    } else {
        println!("NO")
    }
}
