use proconio::*;

struct BitSubset {
    bits: u8,
    state: [usize; usize::BITS as usize],
    mask: [usize; usize::BITS as usize],
}

impl BitSubset {
    /// # Panics
    /// - `(1..=64).contains(&bit)` must be satisfied.
    fn new(bits: u8) -> Self {
        Self {
            bits: bits | (1 << 7),
            state: [0; usize::BITS as usize],
            mask: [0; usize::BITS as usize],
        }
    }
}

impl Iterator for BitSubset {
    type Item = std::iter::Take<std::array::IntoIter<usize, { usize::BITS as usize }>>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.bits > 64 {
            self.bits ^= 1 << 7;
            let mask = 1usize.wrapping_neg() >> (usize::BITS - self.bits as u32);
            self.state[0] = mask;
            self.mask[0] = mask;
            Some(self.state.into_iter().take(1))
        } else {
            for i in (0..self.bits as usize - 1).rev() {
                if self.state[i].count_ones() > 1 {
                    let mut old = self.state[i];
                    let lsb = old & old.wrapping_neg();
                    old ^= lsb;
                    old = (old - 1) & self.mask[i];
                    self.state[i] = old | lsb;
                    self.state[i + 1] = self.mask[i] ^ self.state[i];
                    self.mask[i + 1] = self.state[i + 1];
                    return Some(self.state.into_iter().take(i + 2));
                }
            }
            None
        }
    }
}

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut memo = vec![0; 1 << n];
    for (i, memo) in memo.iter_mut().enumerate() {
        for j in 0..n {
            if i & (1 << j) != 0 {
                *memo += a[j];
            }
        }
    }

    let mut res = vec![];
    let state = BitSubset::new(n as u8);
    for subset in state {
        res.push(subset.fold(0, |s, v| s ^ memo[v]));
    }
    eprintln!("res.len(): {}", res.len());
    res.sort_unstable();
    res.dedup();
    println!("{}", res.len());
}
