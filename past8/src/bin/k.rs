use ac_library::MinCostFlowGraph;
use proconio::*;

fn main() {
    input! {p: usize, q: usize, s: [marker::Bytes; p], ab: [(i64, i64); p], cd: [(i64, i64); q]}

    let mut mcf = MinCostFlowGraph::new(p + q + 2);
    let sp1 = p + q;
    let sp2 = p + q + 1;
    let mut res = 0;
    for i in 0..p {
        res += ab[i].1;
        mcf.add_edge(sp1, i, 1, 0);
    }
    for i in 0..q {
        res += cd[i].1;
        mcf.add_edge(p + i, sp2, 1, 0);
    }

    const M: i64 = i64::MAX >> 20;
    for (i, &(a, b)) in ab.iter().enumerate() {
        for (j, &(c, d)) in cd.iter().enumerate() {
            let bnf = a + c - b - d;
            if bnf > 0 && s[i][j] == b'1' {
                mcf.add_edge(i, p + j, 1, M - bnf);
            }
        }
    }

    println!(
        "{}",
        mcf.slope(sp1, sp2, (p + q) as i64)
            .into_iter()
            .map(|(f, s)| res + M * f - s)
            .max()
            .unwrap_or(res)
    );
}
