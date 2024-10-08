#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    source::line::LineSource,
    *,
};

#[fastout]
fn main() {
    input! {n: usize, s: String, mut t: String}

    let s = t
        .chars()
        .chain(t.chars())
        .chain((0..2 * n).map(|_| 'z'))
        .chain(s.chars())
        .chain(s.chars())
        .collect::<String>();

    let sa = suffixarray::SuffixArray::new(s);
    let sa = sa.get_sa();

    let mut res = 0;
    let mut ts = 0;
    for index in sa
        .iter()
        .filter(|index| **index < n || (4 * n <= **index && **index < 5 * n))
    {
        if *index < n {
            ts += 1;
        } else {
            res += ts;
        }
    }

    println!("{}", n * n - res);
}

pub mod suffixarray {
    ////////////////////////////////////////////////////////////////////////////////
    // Suffix Array
    ////////////////////////////////////////////////////////////////////////////////

    // s[i] := A suffix of the string 's' that begins from i-th element. (0 <= i < s.len()-1)
    //      S   : s[i] < s[i+1]
    //      L   : s[i] > s[i+1]
    //      LMS : s[i-1] = L-type && s[i] = S-type (Left-Most-S)
    #[derive(Debug, Clone, Copy, PartialEq)]
    enum Type {
        S,
        L,
        LMS,
    }

    pub struct SuffixArray {
        s: Vec<usize>,
        sa: Vec<usize>,
    }

    impl SuffixArray {
        const CHARS: usize = 1 << 8;

        pub fn new(s: impl Into<String>) -> Self {
            let s = [s.into().bytes().map(|b| b as usize).collect(), vec![0]].concat();
            let mut sa = vec![std::usize::MAX; s.len()];
            Self::sa_is(Self::CHARS, &s, &mut sa);

            Self { s, sa }
        }

        fn sa_is(kinds: usize, s: &[usize], sa: &mut Vec<usize>) {
            let mut types = vec![Type::S; s.len()];
            let mut lms_indices = vec![vec![]; kinds];
            let mut char_num = vec![0; kinds];

            for (i, c) in s.iter().enumerate().rev() {
                char_num[*c] += 1;

                if i == s.len() - 1 {
                    continue;
                }

                let nc = &s[i + 1];
                types[i] = if c < nc {
                    Type::S
                } else if c > nc {
                    if types[i + 1] == Type::S {
                        types[i + 1] = Type::LMS;
                        lms_indices[*nc].push(i + 1);
                    }
                    Type::L
                } else {
                    // if s[i+1] := x, and s[i] := ax := a{s[i+1]}
                    // if c != nc, then x[0] == a. so, s[i+1] := x := a{s[i+2]}
                    // so s[i] <> s[i+1] == a{s[i+1]} <> a{s[i+2]}
                    //      if s[i+1] < s[i+2] (types[i+1] = S), then s[i] < s[i+1] and types[i] = S
                    //      otherwise s[i+1] > s[i+2] (types[i+1] = L), then s[i] > s[i+1] and types[i] = L
                    types[i + 1]
                }
            }

            let char_start = [
                vec![0],
                char_num
                    .iter()
                    .scan(0, |s, n| {
                        *s += *n;
                        Some(*s)
                    })
                    .collect(),
            ]
            .concat();

            // Calculate Pseudo SA
            Self::induced_sort(
                &lms_indices.into_iter().flatten().collect::<Vec<_>>(),
                &char_start,
                &char_num,
                s,
                &types,
                sa,
            );

            let mut rank = 0;
            let mut lms_prev = (std::usize::MAX, std::usize::MAX);
            let mut lms_perm = vec![];
            let mut lms_indices = vec![std::usize::MAX; s.len()];
            for index in sa
                .iter()
                .take(s.len())
                .filter(|index| types[**index] == Type::LMS)
            {
                if lms_prev == (std::usize::MAX, std::usize::MAX) {
                    lms_prev = (*index, *index);
                    lms_indices[*index] = rank;
                    lms_perm.push(*index);
                } else {
                    let (l, mut r) = (*index, *index + 1);
                    while r < types.len() && types[r] != Type::LMS {
                        r += 1;
                    }

                    let (pl, pr) = lms_prev;
                    if pr - pl != r - l || s[pl..pr + 1] != s[l..r + 1] {
                        rank += 1;
                        lms_prev = (l, r);
                    }
                    lms_indices[l] = rank;
                    lms_perm.push(l);
                }
            }

            let lms_indices = if lms_perm.len() == rank + 1 {
                lms_perm
            } else {
                let (restore_index, lms_ranks) = lms_indices
                    .into_iter()
                    .enumerate()
                    .filter(|(_, c)| *c != std::usize::MAX)
                    .unzip::<usize, usize, Vec<usize>, Vec<usize>>();
                Self::sa_is(rank + 1, &lms_ranks, sa);
                sa.iter_mut()
                    .take(lms_ranks.len())
                    .map(|i| restore_index[*i])
                    .collect()
            };

            Self::induced_sort(&lms_indices, &char_start, &char_num, s, &types, sa);
        }

        fn induced_sort(
            lms_indices: &[usize],
            char_start: &[usize],
            char_num: &[usize],
            s: &[usize],
            types: &[Type],
            sa: &mut [usize],
        ) {
            let kinds = char_start.len();
            sa.iter_mut()
                .take(s.len())
                .for_each(|sa| *sa = std::usize::MAX);
            sa[0] = s.len() - 1;

            let mut checked = 1;
            let mut filled = vec![0; kinds];
            for (i, lms) in lms_indices.iter().enumerate() {
                let c = s[*lms];

                if i > 0 && c != s[lms_indices[i - 1]] {
                    let target = char_start[c] + char_num[c] - 1;

                    while checked <= target {
                        if sa[checked] != std::usize::MAX
                            && sa[checked] > 0
                            && types[sa[checked] - 1] == Type::L
                        {
                            let nc = s[sa[checked] - 1];
                            sa[char_start[nc] + filled[nc]] = sa[checked] - 1;
                            filled[nc] += 1;
                        }

                        checked += 1;
                    }
                }

                if types[*lms - 1] == Type::L {
                    let nc = s[*lms - 1];
                    sa[char_start[nc] + filled[nc]] = *lms - 1;
                    filled[nc] += 1;
                }
            }

            for i in checked..s.len() {
                if sa[i] != std::usize::MAX && sa[i] > 0 && types[sa[i] - 1] == Type::L {
                    let c = s[sa[i] - 1];
                    sa[char_start[c] + filled[c]] = sa[i] - 1;
                    filled[c] += 1;

                    if i != 0 && sa[i] != std::usize::MAX && types[sa[i]] != Type::L {
                        sa[i] = std::usize::MAX;
                    }
                }
            }

            let mut filled = vec![0; kinds];
            for i in (0..s.len()).rev() {
                if sa[i] != std::usize::MAX && sa[i] > 0 && types[sa[i] - 1] != Type::L {
                    let c = s[sa[i] - 1];
                    sa[char_start[c] + char_num[c] - 1 - filled[c]] = sa[i] - 1;
                    filled[c] += 1;
                }
            }
        }

        pub fn get_sa(&self) -> &[usize] {
            &self.sa[1..]
        }

        /// LCPA\[i\] := Longest Common Prefix between s\[sa\[i\]\] and s\[sa\[i+1\]\]
        pub fn lcp_array(&self) -> Vec<usize> {
            let mut rank = vec![0; self.sa.len() - 1];
            for (i, sa) in self.sa.iter().enumerate().skip(1) {
                rank[*sa] = i;
            }

            let mut lcp = 0;
            let mut lcpa = vec![0; self.sa.len() - 2];
            for index in rank {
                if index == self.sa.len() - 1 {
                    lcp = 0;
                    continue;
                }

                let (pos_l, pos_r) = (self.sa[index], self.sa[index + 1]);
                while self.s[lcp + pos_l] == self.s[lcp + pos_r] {
                    lcp += 1;
                }
                lcpa[index - 1] = lcp;

                lcp = lcp.saturating_sub(1);
            }
            lcpa
        }
    }
}
