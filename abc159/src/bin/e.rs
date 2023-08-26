use proconio::*;

fn main() {
    input! {h: usize, w: usize, k: usize, s: [marker::Chars; h]}

    let mut res = std::u32::MAX;
    for i in 0..(1u32 << (h - 1)) {
        let row = i.count_ones() as usize + 1;
        let mut buf = vec![vec![0; w]; row];
        let mut pc = (0..h - 1)
            .filter(|&j| i & (1 << j) != 0)
            .map(|v| v + 1)
            .collect::<Vec<_>>();
        pc.insert(0, 0);
        pc.push(h);
        for j in 0..w {
            for (i, v) in pc.windows(2).enumerate() {
                for k in v[0]..v[1] {
                    buf[i][j] += (s[k][j] == '1') as usize;
                }
            }
        }

        // eprintln!("i: {i:0b}, buf: {buf:?}");

        if buf.iter().flatten().any(|&v| v > k) {
            continue;
        }

        let mut sum = i.count_ones();
        let mut t = vec![0; row];
        for i in 0..w {
            for j in 0..row {
                t[j] += buf[j][i];
            }
            if t.iter().any(|&t| t > k) {
                sum += 1;
                for j in 0..row {
                    t[j] = buf[j][i];
                }
            }
        }

        // eprintln!("sum: {sum}");

        res = res.min(sum);
    }

    println!("{}", res)
}
