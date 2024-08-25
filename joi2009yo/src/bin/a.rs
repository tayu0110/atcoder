use proconio::*;

fn main() {
    input! {a: [[[usize; 3]; 2]; 3]}

    for a in a {
        let [mut h, mut m, mut s, ..] = a[0][..] else {
            unreachable!()
        };

        let mut res = 0;
        'b: loop {
            while m < 60 {
                while s < 60 {
                    if [h, m, s] == a[1][..] {
                        break 'b;
                    }

                    s += 1;
                    res += 1;
                }

                s = 0;
                m += 1;
            }

            m = 0;
            h += 1;
        }

        let s = res % 60;
        let m = (res / 60) % 60;
        let h = (res / 60) / 60;
        println!("{h} {m} {s}")
    }
}
