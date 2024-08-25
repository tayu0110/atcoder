use std::{io::Write, ops::Index, time::Instant};

use ordered_float::OrderedFloat;
use proconio::*;
use rustc_hash::FxHashSet;

const MAX_M: usize = 20;

#[derive(Debug, Clone, Copy)]
struct Memo {
    n: usize,
    field: [i8; MAX_M * MAX_M],
    checked: [bool; MAX_M * MAX_M],
}

impl Memo {
    fn new(n: usize) -> Self {
        Self {
            n,
            field: [0; MAX_M * MAX_M],
            checked: [false; MAX_M * MAX_M],
        }
    }

    fn is_checked(&self, r: usize, c: usize) -> bool {
        self.checked[r * self.n + c]
    }

    fn is_ng(&self, r: usize, c: usize) -> bool {
        self.checked[r * self.n + c] && self.field[r * self.n + c] == 0
    }

    fn is_ok(&self, r: usize, c: usize) -> bool {
        self.field[r * self.n + c] != 0
    }

    fn set_and_check(&mut self, r: usize, c: usize, val: usize) {
        self.checked[r * self.n + c] = true;
        self.field[r * self.n + c] = val as i8;
    }

    fn check(&mut self, r: usize, c: usize, val: usize) {
        if self.is_checked(r, c) {
            return;
        }

        self.checked[r * self.n + c] = true;
        self.field[r * self.n + c] += val as i8;
        assert!(self.field[r * self.n + c] >= 0);
    }

    fn iter(&self) -> impl Iterator<Item = i8> + '_ {
        self.field
            .iter()
            .take(self.n * self.n)
            .zip(self.checked.iter().take(self.n * self.n))
            .map(|(f, c)| if *c { *f } else { i8::MAX })
    }

    fn decide(
        &mut self,
        index: usize,
        good: &mut Vec<FxHashSet<usize>>,
        panel: &mut Vec<Vec<(usize, usize)>>,
        res: &mut Vec<(usize, usize)>,
    ) {
        assert_eq!(good[index].len(), 1);
        let id = good[index].iter().next().unwrap();
        let (r, c) = (id / self.n, id % self.n);
        for (i, j) in &panel[index] {
            let (r, c) = (r + i, c + j);
            self.field[r * self.n + c] -= 1;
            res.push((r, c));
        }
        good.swap_remove(index);
        panel.swap_remove(index);
    }
}

impl Index<(usize, usize)> for Memo {
    type Output = i8;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (r, c) = index;
        &self.field[r * self.n + c]
    }
}

fn query(i: usize, j: usize) -> usize {
    println!("q 1 {i} {j}");
    std::io::stdout().flush().unwrap();
    input_interactive!(v: usize);
    v
}

fn verify_exactly_2(
    n: usize,
    panel: &[Vec<(usize, usize)>],
    memo: &mut Memo,
    good: &[FxHashSet<usize>],
    black_list: &mut FxHashSet<(usize, usize, usize, usize)>,
) {
    assert_eq!(panel.len(), 2);
    let mut t = vec![vec![0; n]; n];
    let mut can_use = vec![vec![false; n]; n];
    for p in good[0].iter() {
        let (r, c) = (p / n, p % n);
        for p2 in good[1].iter() {
            let (r2, c2) = (p2 / n, p2 % n);
            if black_list.contains(&(r, c, r2, c2)) {
                continue;
            }
            t.iter_mut().for_each(|t| t.fill(0));
            for (r, c, panel) in [(r, c, &panel[0]), (r2, c2, &panel[1])] {
                for &(i, j) in panel {
                    t[r + i][c + j] += 1;
                }
            }
            if t.iter()
                .flatten()
                .zip(memo.iter())
                .filter(|v| v.1 < i8::MAX)
                .any(|(t, m)| *t != m)
            {
                black_list.insert((r, c, r2, c2));
            } else {
                for i in 0..n {
                    for j in 0..n {
                        can_use[i][j] |= t[i][j] > 0;
                    }
                }
            }
        }
    }

    for i in 0..n {
        for j in 0..n {
            if !can_use[i][j] && !memo.is_checked(i, j) {
                memo.check(i, j, 0);
            }
        }
    }
}

fn determinstic_solve_m_equal_to_2(
    n: usize,
    m: usize,
    area_sum: usize,
    panel: Vec<Vec<(usize, usize)>>,
    guess: &mut [Vec<FxHashSet<usize>>],
    memo: &mut Memo,
    good: &mut [FxHashSet<usize>],
    rem: &mut Vec<(usize, usize)>,
    res: &mut Vec<(usize, usize)>,
) {
    assert!(m == 2);

    let mut found_area = 0;
    let mut black_list = FxHashSet::default();
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                for l in 0..n {
                    if !good[0].contains(&(i * n + j)) || !good[1].contains(&(k * n + l)) {
                        black_list.insert((i, j, k, l));
                    }
                }
            }
        }
    }
    verify_exactly_2(n, &panel, memo, good, &mut black_list);

    while let Some((i, j)) = rem.pop() {
        if !memo.is_checked(i, j) {
            let v = query(i, j);
            memo.check(i, j, v);
            found_area += v;

            if v > 0 {
                res.push((i, j));
            }

            if found_area == area_sum {
                break;
            }

            verify_exactly_2(n, &panel, memo, good, &mut black_list);

            {
                let mut b = [0; 4];
                for (s, t) in [(0, 1), (1, 0)] {
                    let mut buf = vec![];
                    'b: for (i, j) in good[s].iter().map(|p| (p / n, p % n)) {
                        for (k, l) in good[t].iter().map(|p| (p / n, p % n)) {
                            b[2 * s] = i;
                            b[2 * s + 1] = j;
                            b[2 * t] = k;
                            b[2 * t + 1] = l;
                            if !black_list.contains(&(b[0], b[1], b[2], b[3])) {
                                continue 'b;
                            }
                        }
                        buf.push((i, j));
                    }
                    for (i, j) in buf {
                        good[s].remove(&(i * n + j));
                    }
                }
            }

            if black_list.len() == n * n * n * n - 1 {
                for &p in &good[0] {
                    let (i, j) = (p / n, p % n);
                    for &p in &good[1] {
                        let (k, l) = (p / n, p % n);
                        if !black_list.contains(&(i, j, k, l)) {
                            for (i, j, p) in [(i, j, &panel[0]), (k, l, &panel[1])] {
                                for (r, c) in p {
                                    res.push((r + i, c + j));
                                }
                            }
                        }
                    }
                }
                break;
            }

            guess.iter_mut().flatten().for_each(FxHashSet::clear);
            for (i, co) in panel.iter().enumerate() {
                let g = good[i].iter().copied().collect::<Vec<_>>();
                for (r, c) in g.into_iter().map(|g| (g / n, g % n)) {
                    if puton(i, r, c, co, guess, memo).is_none() {
                        good[i].remove(&(r * n + c));
                    }
                }
            }
            for i in 0..n {
                for j in 0..n {
                    if guess[i][j].is_empty() && !memo.is_checked(i, j) {
                        memo.check(i, j, 0);
                    }
                }
            }

            rem.sort_unstable_by_key(|&(i, j)| {
                (
                    guess[i][j].len(),
                    i.min(n - 1 - i) + j.min(n - 1 - j),
                    (i + j) & 1,
                )
            });
        }
    }

    res.sort_unstable();
    res.dedup();

    print!("a {}", res.len());
    for (i, j) in res {
        print!(" {i} {j}");
    }
    println!();
    std::io::stdout().flush().unwrap();
}

fn verify_exactly_3(
    n: usize,
    panel: &[Vec<(usize, usize)>],
    memo: &mut Memo,
    white_list: &mut FxHashSet<u32>,
) -> Vec<Vec<i8>> {
    assert_eq!(panel.len(), 3);
    let mut t = vec![vec![0; n]; n];
    let mut can_use = vec![vec![0i8; n]; n];

    let restore_id = |mut id: u32| {
        let mut res = [0; 6];
        for i in (0..6).rev() {
            res[i] = (id & 0b11111) as usize;
            id >>= 5;
        }
        res
    };

    let mut black = vec![];
    for id in white_list.iter().copied() {
        let [r, c, r2, c2, r3, c3] = restore_id(id);
        t.iter_mut().for_each(|t| t.fill(0));
        for (r, c, panel) in [(r, c, &panel[0]), (r2, c2, &panel[1]), (r3, c3, &panel[2])] {
            for &(i, j) in panel {
                t[r + i][c + j] += 1;
            }
        }
        if t.iter()
            .flatten()
            .zip(memo.iter())
            .filter(|v| v.1 < i8::MAX)
            .any(|(t, m)| *t != m)
        {
            black.push(id);
        } else {
            for i in 0..n {
                for j in 0..n {
                    can_use[i][j] = can_use[i][j].saturating_add(t[i][j]);
                }
            }
        }
    }
    for b in black {
        white_list.remove(&b);
    }

    for i in 0..n {
        for j in 0..n {
            if can_use[i][j] == 0 && !memo.is_checked(i, j) {
                memo.check(i, j, 0);
            }
        }
    }
    can_use
}

fn determinstic_solve_m_equal_to_3(
    n: usize,
    m: usize,
    area_sum: usize,
    panel: Vec<Vec<(usize, usize)>>,
    memo: &mut Memo,
    good: &mut [FxHashSet<usize>],
    rem: &mut Vec<(usize, usize)>,
    res: &mut Vec<(usize, usize)>,
    guess: &mut [Vec<FxHashSet<usize>>],
) {
    assert!(m == 3);

    let gen_id =
        |v: [usize; 6]| -> u32 { v.into_iter().fold(0usize, |s, v| s.wrapping_shl(5) | v) as u32 };
    let restore_id = |mut id: u32| {
        let mut res = [0; 6];
        for i in (0..6).rev() {
            res[i] = (id & 0b11111) as usize;
            id >>= 5;
        }
        res
    };

    let mut found_area = 0;
    let mut white_list = FxHashSet::default();
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                for l in 0..n {
                    for m in 0..n {
                        for o in 0..n {
                            if good[0].contains(&(i * n + j))
                                && good[1].contains(&(k * n + l))
                                && good[2].contains(&(m * n + o))
                            {
                                white_list.insert(gen_id([i, j, k, l, m, o]));
                            }
                        }
                    }
                }
            }
        }
    }
    verify_exactly_3(n, &panel, memo, &mut white_list);

    while let Some((i, j)) = rem.pop() {
        if !memo.is_checked(i, j) {
            let v = query(i, j);
            memo.check(i, j, v);
            found_area += v;

            if v > 0 {
                res.push((i, j));
            }

            if found_area == area_sum {
                break;
            }

            verify_exactly_3(n, &panel, memo, &mut white_list);

            {
                let mut b = [0; 6];
                for (s, t, u) in [(0, 1, 2), (1, 2, 0), (2, 0, 1)] {
                    let mut buf = vec![];
                    'b: for (i, j) in good[s].iter().map(|p| (p / n, p % n)) {
                        for (k, l) in good[t].iter().map(|p| (p / n, p % n)) {
                            for (m, n) in good[u].iter().map(|p| (p / n, p % n)) {
                                b[2 * s] = i;
                                b[2 * s + 1] = j;
                                b[2 * t] = k;
                                b[2 * t + 1] = l;
                                b[2 * u] = m;
                                b[2 * u + 1] = n;
                                if white_list.contains(&gen_id(b)) {
                                    continue 'b;
                                }
                            }
                        }
                        buf.push((i, j));
                    }
                    for (i, j) in buf {
                        good[s].remove(&(i * n + j));
                    }
                }
            }

            if good.iter().all(|g| g.len() == 1) {
                let (i, j) = good[0].iter().next().map(|p| (p / n, p % n)).unwrap();
                let (k, l) = good[1].iter().next().map(|p| (p / n, p % n)).unwrap();
                let (m, o) = good[2].iter().next().map(|p| (p / n, p % n)).unwrap();
                for (i, j, p) in [(i, j, &panel[0]), (k, l, &panel[1]), (m, o, &panel[2])] {
                    for (r, c) in p {
                        res.push((r + i, c + j));
                    }
                }
                break;
            }

            if white_list.len() == 1 {
                let &id = white_list.iter().next().unwrap();
                let [i, j, k, l, m, o] = restore_id(id);
                for (i, j, p) in [(i, j, &panel[0]), (k, l, &panel[1]), (m, o, &panel[2])] {
                    for (r, c) in p {
                        res.push((r + i, c + j));
                    }
                }
                break;
            }

            guess.iter_mut().flatten().for_each(FxHashSet::clear);
            for (i, co) in panel.iter().enumerate() {
                let g = good[i].iter().copied().collect::<Vec<_>>();
                for (r, c) in g.into_iter().map(|g| (g / n, g % n)) {
                    if puton(i, r, c, co, guess, memo).is_none() {
                        good[i].remove(&(r * n + c));
                    }
                }
            }
            for i in 0..n {
                for j in 0..n {
                    if guess[i][j].is_empty() && !memo.is_checked(i, j) {
                        memo.check(i, j, 0);
                    }
                }
            }

            rem.sort_unstable_by_key(|&(i, j)| {
                (
                    guess[i][j].len(),
                    i.min(n - 1 - i) + j.min(n - 1 - j),
                    (i + j) & 1,
                )
            });
        }
    }

    res.sort_unstable();
    res.dedup();

    print!("a {}", res.len());
    for (i, j) in res {
        print!(" {i} {j}");
    }
    println!();
    std::io::stdout().flush().unwrap();
}

fn gen_id(v: &[usize]) -> (u128, u128) {
    let mid = v.len() >> 1;
    (
        v[..mid]
            .into_iter()
            .fold(0u128, |s, &v| s.wrapping_shl(5) | v as u128),
        v[mid..]
            .into_iter()
            .fold(0u128, |s, &v| s.wrapping_shl(5) | v as u128),
    )
}

fn restore_id(mut id0: u128, mut id1: u128, m: usize) -> Vec<usize> {
    let mut res = vec![0; 2 * m];
    for i in (0..m).rev() {
        res[i] = (id0 & 0b11111) as usize;
        id0 >>= 5;
    }
    for i in (m..2 * m).rev() {
        res[i] = (id1 & 0b11111) as usize;
        id1 >>= 5;
    }
    res
}

const TM_THRESH: u128 = 2850;

fn verify_exactly(
    n: usize,
    panel: &[Vec<(usize, usize)>],
    memo: &mut Memo,
    white_list: &mut FxHashSet<(u128, u128)>,
    tm: &Instant,
) -> Vec<Vec<i8>> {
    let mut t = vec![vec![0; n]; n];
    let mut can_use = vec![vec![0i8; n]; n];

    let mut black = vec![];
    'b: for id in white_list.iter().copied() {
        let v = restore_id(id.0, id.1, panel.len());
        t.iter_mut().for_each(|t| t.fill(0));
        for (v, panel) in v.chunks_exact(2).zip(panel) {
            let (r, c) = (v[0], v[1]);
            for (i, j) in panel {
                t[r + i][c + j] += 1;
                if memo.is_checked(r + i, c + j) && t[r + i][c + j] > memo[(r + i, c + j)] {
                    black.push(id);
                    continue 'b;
                }
            }
        }
        if t.iter()
            .flatten()
            .zip(memo.iter())
            .filter(|v| v.1 < i8::MAX)
            .any(|(t, m)| *t != m)
        {
            black.push(id);
        } else {
            for i in 0..n {
                for j in 0..n {
                    can_use[i][j] = can_use[i][j].saturating_add(t[i][j]);
                }
            }
        }
        if tm.elapsed().as_millis() > TM_THRESH {
            return can_use;
        }
    }
    for b in black {
        white_list.remove(&b);
    }

    for i in 0..n {
        for j in 0..n {
            if can_use[i][j] == 0 && !memo.is_checked(i, j) {
                memo.set_and_check(i, j, 0);
            }
        }
    }
    can_use
}

fn determinstic_solve(
    n: usize,
    area_sum: usize,
    panel: &[Vec<(usize, usize)>],
    memo: &mut Memo,
    good: &mut [FxHashSet<usize>],
    rem: &mut Vec<(usize, usize)>,
    res: &mut Vec<(usize, usize)>,
    guess: &mut [Vec<FxHashSet<usize>>],
    tm: &Instant,
) -> bool {
    fn enumerate_white_list(
        n: usize,
        good: &[FxHashSet<usize>],
        white_list: &mut FxHashSet<(u128, u128)>,
        list: &mut Vec<usize>,
        tm: &Instant,
    ) -> bool {
        if good.is_empty() {
            white_list.insert(gen_id(list));
            return true;
        }

        for &p in &good[0] {
            let (r, c) = (p / n, p % n);
            list.push(r);
            list.push(c);
            let r = enumerate_white_list(n, &good[1..], white_list, list, tm);
            list.pop();
            list.pop();
            if tm.elapsed().as_millis() > TM_THRESH || !r {
                return false;
            }
        }

        true
    }

    let mut found_area = 0;
    let mut white_list = FxHashSet::default();
    if !enumerate_white_list(n, good, &mut white_list, &mut vec![], tm) {
        return false;
    }
    verify_exactly(n, &panel, memo, &mut white_list, tm);
    if tm.elapsed().as_millis() > TM_THRESH {
        return false;
    }

    let mut prior = vec![vec![0u8; n]; n];
    while let Some((i, j)) = rem.pop() {
        if !memo.is_checked(i, j) {
            let v = query(i, j);
            memo.check(i, j, v);
            found_area += v;

            if v > 0 {
                res.push((i, j));
            }

            if found_area == area_sum {
                break;
            }

            verify_exactly(n, &panel, memo, &mut white_list, tm);
            if tm.elapsed().as_millis() > TM_THRESH {
                return false;
            }

            {
                let mut perm = (0..good.len()).collect::<Vec<_>>();
                for i in 0..good.len() {
                    perm.swap(0, i);
                    reduce_goods(
                        n,
                        true,
                        &perm,
                        good,
                        &white_list,
                        &mut vec![0; 2 * good.len()],
                        tm,
                    );
                    if tm.elapsed().as_millis() > TM_THRESH {
                        return false;
                    }
                }

                fn reduce_goods(
                    n: usize,
                    top: bool,
                    perm: &[usize],
                    good: &mut [FxHashSet<usize>],
                    white_list: &FxHashSet<(u128, u128)>,
                    buf: &mut [usize],
                    tm: &Instant,
                ) -> bool {
                    if perm.is_empty() {
                        return !white_list.contains(&gen_id(buf));
                    }

                    let mut remove = vec![];
                    let mut f = true;
                    let goods = good[perm[0]]
                        .iter()
                        .map(|p| (p / n, p % n))
                        .collect::<Vec<_>>();
                    for (i, j) in goods {
                        buf[2 * perm[0]] = i;
                        buf[2 * perm[0] + 1] = j;
                        let res = reduce_goods(n, false, &perm[1..], good, white_list, buf, tm);
                        if tm.elapsed().as_millis() > TM_THRESH {
                            return false;
                        }
                        if top && res {
                            remove.push((i, j));
                        }
                        f &= res;
                        if !top && !f {
                            break;
                        }
                    }

                    for (i, j) in remove {
                        good[perm[0]].remove(&(i * n + j));
                    }

                    f
                }
            }

            if good.iter().all(|g| g.len() == 1) {
                for k in 0..good.len() {
                    let (i, j) = good[k].iter().next().map(|p| (p / n, p % n)).unwrap();
                    for (r, c) in &panel[k] {
                        res.push((r + i, c + j));
                    }
                }
                break;
            }

            if white_list.len() == 1 {
                let &id = white_list.iter().next().unwrap();
                let v = restore_id(id.0, id.1, good.len());
                for (v, p) in v.chunks_exact(2).zip(panel) {
                    for (r, c) in p {
                        res.push((r + v[0], c + v[1]));
                    }
                }
                break;
            }

            shrink_good(n, guess, panel, good, memo, rem);

            for &(i, j) in rem.iter() {
                let mut p = 0;
                for i in i.saturating_sub(1)..(i + 2).min(n) {
                    for j in j.saturating_sub(1)..(j + 2).min(n) {
                        p += (!memo.is_checked(i, j)) as u8;
                    }
                }
                prior[i][j] = p;
            }
            rem.sort_unstable_by_key(|&(i, j)| (guess[i][j].len(), prior[i][j]));
        }
        if tm.elapsed().as_millis() > TM_THRESH {
            return false;
        }
    }

    res.sort_unstable();
    res.dedup();

    print!("a {}", res.len());
    for (i, j) in res {
        print!(" {i} {j}");
    }
    println!();
    std::io::stdout().flush().unwrap();
    true
}

fn puton(
    index: usize,
    r: usize,
    c: usize,
    co: &[(usize, usize)],
    guess: &mut [Vec<FxHashSet<usize>>],
    memo: &mut Memo,
) -> Option<()> {
    let n = guess.len();
    for (i, j) in co {
        if r + i >= n || c + j >= n {
            return None;
        }
        if memo.is_ng(r + i, c + j) {
            return None;
        }
    }
    for (i, j) in co {
        guess[r + i][c + j].insert(index);
    }
    Some(())
}

fn shrink_good(
    n: usize,
    guess: &mut [Vec<FxHashSet<usize>>],
    panel: &[Vec<(usize, usize)>],
    good: &mut [FxHashSet<usize>],
    memo: &mut Memo,
    rem: &[(usize, usize)],
) {
    guess.iter_mut().flatten().for_each(FxHashSet::clear);
    for (i, co) in panel.iter().enumerate() {
        let g = good[i].iter().copied().collect::<Vec<_>>();
        for (r, c) in g.into_iter().map(|g| (g / n, g % n)) {
            if puton(i, r, c, co, guess, memo).is_none() {
                good[i].remove(&(r * n + c));
            }
        }
    }
    for &(i, j) in rem.iter() {
        if guess[i][j].is_empty() && !memo.is_checked(i, j) {
            memo.set_and_check(i, j, 0);
        }
    }
}

fn main() {
    let tm = Instant::now();
    input_interactive!(n: usize, m: usize, _: f64);

    let mut area_sum = 0;
    let mut panel = vec![];
    let (mut ph, mut pw) = (usize::MAX, usize::MAX);
    for _ in 0..m {
        input_interactive!(d: usize, co: [(usize, usize); d]);
        for &(r, c) in &co {
            ph = ph.min(r);
            pw = pw.min(c);
        }
        panel.push(co);
        area_sum += d;
    }
    ph = ph.max(4);
    pw = pw.max(4);

    let mut guess = vec![vec![FxHashSet::default(); n]; n];
    let mut memo = Memo::new(n);
    let mut good = vec![FxHashSet::default(); m];
    for (i, co) in panel.iter().enumerate() {
        for r in 0..n {
            for c in 0..n {
                if puton(i, r, c, co, &mut guess, &mut memo).is_some() {
                    good[i].insert(r * n + c);
                }
            }
        }
    }

    let mut rem = vec![];
    for i in 0..n {
        for j in 0..n {
            if guess[i][j].is_empty() {
                memo.check(i, j, 0);
            } else {
                rem.push((i, j));
            }
        }
    }
    rem.sort_unstable_by_key(|&(i, j)| {
        (
            guess[i][j].len(),
            i.min(n - 1 - i) + j.min(n - 1 - j),
            (i + j) & 1,
        )
    });

    let mut found_area = 0;
    let mut res = vec![];
    'b: for i in (0..n).step_by(ph) {
        for j in (i % ph..n).step_by(pw) {
            if !memo.is_checked(i, j) {
                let v = query(i, j);
                found_area += v;
                if found_area == area_sum {
                    break 'b;
                }
                memo.check(i, j, v);
                if v > 0 {
                    res.push((i, j));
                } else {
                    guess.iter_mut().flatten().for_each(FxHashSet::clear);
                    for (i, co) in panel.iter().enumerate() {
                        let g = good[i].iter().copied().collect::<Vec<_>>();
                        for (r, c) in g.into_iter().map(|g| (g / n, g % n)) {
                            if puton(i, r, c, co, &mut guess, &mut memo).is_none() {
                                good[i].remove(&(r * n + c));
                            }
                        }
                    }
                    for i in 0..n {
                        for j in 0..n {
                            if guess[i][j].is_empty() && !memo.is_checked(i, j) {
                                memo.check(i, j, 0);
                            }
                        }
                    }
                }
            }
        }
    }

    rem.sort_unstable_by_key(|&(i, j)| {
        (
            guess[i][j].len(),
            i.min(n - 1 - i) + j.min(n - 1 - j),
            (i + j) & 1,
        )
    });
    if m == 2 {
        determinstic_solve_m_equal_to_2(
            n,
            m,
            area_sum - found_area,
            panel,
            &mut guess,
            &mut memo,
            &mut good,
            &mut rem,
            &mut res,
        );
        return;
    }

    if m == 3 {
        determinstic_solve_m_equal_to_3(
            n,
            m,
            area_sum - found_area,
            panel,
            &mut memo,
            &mut good,
            &mut rem,
            &mut res,
            &mut guess,
        );
        return;
    }

    const DET_THRESH: usize = 4000000;
    let mut use_det = true;

    if use_det
        && good
            .iter()
            .map(|g| g.len())
            .fold(1usize, |s, g| s.saturating_mul(g))
            <= DET_THRESH
    {
        use_det = determinstic_solve(
            n,
            area_sum - found_area,
            &panel,
            &mut memo,
            &mut good,
            &mut rem,
            &mut res,
            &mut guess,
            &tm,
        );
        // eprintln!("tm: {} msec", tm.elapsed().as_millis());
        if use_det {
            return;
        }
    }

    let mut prior = vec![vec![0u8; n]; n];
    let mut penalty = vec![vec![0.0f32; n]; n];
    while let Some((i, j)) = rem.pop() {
        if !memo.is_checked(i, j) {
            let v = query(i, j);
            memo.check(i, j, v);
            if v > 0 {
                res.push((i, j));
                found_area += v;
                if found_area == area_sum {
                    break;
                }
            } else {
                shrink_good(n, &mut guess, &panel, &mut good, &mut memo, &rem);
            }
        }
        for &(i, j) in &rem {
            let mut p = 0;
            let mut pen = 0.0;
            let mut c = 0;
            for i in i.saturating_sub(1)..(i + 2).min(n) {
                for j in j.saturating_sub(1)..(j + 2).min(n) {
                    p += (!memo.is_checked(i, j)) as u8;
                    pen += memo.is_ng(i, j) as u8 as f32;
                    c += 1;
                }
            }
            prior[i][j] = p;
            penalty[i][j] = pen / c as f32;
        }

        {
            let mut cnt = 0;
            while cnt < good.len() {
                if good[cnt].len() == 1 && !panel[cnt].is_empty() {
                    memo.decide(cnt, &mut good, &mut panel, &mut res);
                    continue;
                }
                cnt += 1;
            }
        }

        rem.sort_unstable_by_key(|&(i, j)| {
            (
                (memo.is_ok(i, j) && !memo.is_checked(i, j)),
                guess[i][j].len(),
                prior[i][j],
                -OrderedFloat::from(penalty[i][j]),
            )
        });

        if use_det
            && good
                .iter()
                .map(|g| g.len())
                .fold(1usize, |s, g| s.saturating_mul(g))
                <= DET_THRESH
        {
            use_det = determinstic_solve(
                n,
                area_sum - found_area,
                &panel,
                &mut memo,
                &mut good,
                &mut rem,
                &mut res,
                &mut guess,
                &tm,
            );
            // eprintln!("tm: {} msec", tm.elapsed().as_millis());
            if use_det {
                return;
            }
        }
    }

    res.sort_unstable();
    res.dedup();
    print!("a {}", res.len());
    for (i, j) in res {
        print!(" {i} {j}");
    }
    println!();
    std::io::stdout().flush().unwrap();
}
