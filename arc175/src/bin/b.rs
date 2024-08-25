use proconio::*;
use segtree::{LazySegtree, MapMonoid};

struct S;

impl MapMonoid for S {
    type E = i32;
    type Act = i32;
    fn e() -> Self::E {
        i32::MAX
    }
    fn op(l: Self::E, r: Self::E) -> Self::E {
        l.min(r)
    }

    fn id() -> Self::Act {
        0
    }
    fn map(act: Self::Act, elem: Self::E) -> Self::E {
        act + elem
    }

    fn composite(l: Self::Act, r: Self::Act) -> Self::Act {
        l + r
    }
}

fn f1(s: &[char], _a: usize, b: usize, m: usize) -> (Vec<char>, usize) {
    let mut s = s.to_vec();
    let t = s
        .iter()
        .map(|&s| if s == '(' { 1 } else { -1 })
        .sum::<i32>();
    let mut res = 0;
    if t < 0 {
        let mut k = -t / 2;
        res += k as usize * b;
        for i in 0..m {
            if k > 0 && s[i] == ')' {
                s[i] = '(';
                k -= 1;
            }
        }
    } else if t > 0 {
        let mut k = t / 2;
        res += k as usize * b;
        for i in (0..m).rev() {
            if k > 0 && s[i] == '(' {
                s[i] = ')';
                k -= 1;
            }
        }
    }
    (s, res)
}

fn f2(s: &[char], a: usize, _b: usize, m: usize) -> (Vec<char>, usize) {
    let mut s = s.to_vec();
    let mut t = vec![0];
    for (i, &s) in s.iter().enumerate() {
        let d = t[i] + if s == '(' { 1 } else { -1 };
        t.push(d);
    }

    t.remove(0);

    let mut res = 0;
    let mut st = LazySegtree::<S>::from_vec(&t);
    let (mut l, mut r) = (0, m - 1);
    while st.prod(0, m) < 0 {
        while l < m && st.prod(l, l + 1) >= 0 {
            l += 1;
        }

        if l >= m {
            break;
        }

        assert_eq!(s[l], ')');
        while l < r && s[r] == ')' {
            r -= 1;
        }

        if r <= l {
            break;
        }

        res += a;
        s.swap(l, r);
        st.apply_range(l, r, 2);
    }

    (s, res)
}

type F = dyn Fn(&[char], usize, usize, usize) -> (Vec<char>, usize);

fn main() {
    input! {n: usize, a: usize, b: usize, mut s: marker::Chars}
    let m = 2 * n;
    let a = a.min(b * 2);

    let mut res = usize::MAX;
    for (f, g) in [(&f1 as &F, &f2 as &F), (&f2, &f1)] {
        let (s, r) = f(&s, a, b, m);
        let (_, s) = g(&s, a, b, m);
        res = res.min(r + s);
    }

    println!("{res}")
}
