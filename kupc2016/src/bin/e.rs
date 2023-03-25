use flow::Dinic;
use proconio::{input, marker::Chars};

fn main() {
    input! {h: usize, w: usize, s:[Chars; h]}

    // in: i, out: i+h*w, src: 2*h*w, dst: 2*h*w+1
    let get_index = |i: usize, j: usize| i * w + j;
    let (src, dst) = (2 * h * w, 2 * h * w + 1);
    let mut ff = Dinic::new(h * w * 2 + 2);
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == 'X' {
                if (i == 0 || i == h - 1) && (j == 0 || j == w - 1) {
                    println!("-1");
                    return;
                }

                ff.set_edge(src, get_index(i, j), std::usize::MAX);
                ff.set_edge(get_index(i, j), get_index(i, j) + h * w, std::usize::MAX);
            } else {
                ff.set_edge(get_index(i, j), get_index(i, j) + h * w, 1);
            }
            if i == 0 || i == h - 1 || j == 0 || j == w - 1 {
                ff.set_edge(get_index(i, j) + h * w, dst, std::usize::MAX);
            }

            if i > 0 {
                ff.set_edge(
                    get_index(i, j) + h * w,
                    get_index(i - 1, j),
                    std::usize::MAX,
                );
            }
            if i + 1 < h {
                ff.set_edge(
                    get_index(i, j) + h * w,
                    get_index(i + 1, j),
                    std::usize::MAX,
                );
            }
            if j > 0 {
                ff.set_edge(
                    get_index(i, j) + h * w,
                    get_index(i, j - 1),
                    std::usize::MAX,
                );
            }
            if j + 1 < w {
                ff.set_edge(
                    get_index(i, j) + h * w,
                    get_index(i, j + 1),
                    std::usize::MAX,
                );
            }
        }
    }

    println!("{}", ff.flow(src, dst));
}
