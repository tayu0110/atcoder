use flow::maximum_matching_of_bipartite_graph;
use proconio::input;

fn main() {
    input! {m: usize, n: usize, mut a: [[u8; n]; m], b: [[u8; n]; m]}

    let get_index = |i: usize, j: usize| i * n + j;
    let mut edges = vec![];
    let mut diff = 0;
    for i in 0..m {
        for j in 0..n {
            if a[i][j] == b[i][j] {
                continue;
            }

            if j + 1 < n && a[i][j + 1] != b[i][j + 1] && a[i][j] != a[i][j + 1] {
                if (i + j) % 2 == 0 {
                    edges.push((get_index(i, j), get_index(i, j + 1)));
                } else {
                    edges.push((get_index(i, j + 1), get_index(i, j)));
                }
            }

            if i + 1 < m && a[i + 1][j] != b[i + 1][j] && a[i][j] != a[i + 1][j] {
                if (i + j) % 2 == 0 {
                    edges.push((get_index(i, j), get_index(i + 1, j)));
                } else {
                    edges.push((get_index(i + 1, j), get_index(i, j)));
                }
            }

            diff += 1;
        }
    }

    let matching = maximum_matching_of_bipartite_graph(n * m, edges);

    println!("{}", diff - matching.len());
}
