use rand::Rng;
use std::f64::INFINITY;
use std::io::{self, Write};

#[derive(Debug, Clone)]
struct Paint {
    c: f64, // シアン成分
    m: f64, // マゼンタ成分
    y: f64, // イエロー成分
}

#[derive(Debug, Clone)]
struct Tube {
    color: Paint,
}

#[derive(Debug, Clone)]
struct Target {
    color: Paint,
}

fn read_usize_line() -> Vec<usize> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn read_f64_line() -> Vec<f64> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn euclidean_distance(p1: &Paint, p2: &Paint) -> f64 {
    // ユークリッド距離で色の差を計算
    let dc = p1.c - p2.c;
    let dm = p1.m - p2.m;
    let dy = p1.y - p2.y;
    (dc * dc + dm * dm + dy * dy).sqrt()
}

fn main() {
    // N, K, H, T, D の入力
    let params: Vec<usize> = read_usize_line();
    let n = params[0];
    let k = params[1];
    let h = params[2];
    let t = params[3];
    let d = params[4];

    let mut tubes = Vec::new();
    let mut targets = Vec::new();

    // チューブ絵の具の色を入力
    for _ in 0..k {
        let colors: Vec<f64> = read_f64_line();
        tubes.push(Tube {
            color: Paint {
                c: colors[0],
                m: colors[1],
                y: colors[2],
            },
        });
    }

    // 目標色を入力
    for _ in 0..h {
        let colors: Vec<f64> = read_f64_line();
        targets.push(Target {
            color: Paint {
                c: colors[0],
                m: colors[1],
                y: colors[2],
            },
        });
    }

    // パレットの初期状態（仕切り配置）
    let mut vertical_bars = vec![vec![0; n - 1]; n];
    let mut horizontal_bars = vec![vec![0; n]; n - 1];

    // 仕切りの初期配置を出力
    for i in 0..n {
        for j in 0..(n - 1) {
            print!("{} ", vertical_bars[i][j]);
        }
        println!();
    }

    for i in 0..(n - 1) {
        for j in 0..n {
            print!("{} ", horizontal_bars[i][j]);
        }
        println!();
    }

    // 乱数生成
    let mut rng = rand::thread_rng();

    // 操作1と操作2を交互に出力し、操作2をH回実行する
    let mut operation_count = 0;
    let mut total_operations = 0;

    while operation_count < t && total_operations < h {
        // 目標色に近いチューブを選ぶ
        if operation_count < t {
            let target_color = &targets[total_operations].color;
            let mut min_distance = INFINITY;
            let mut selected_tube_index = 0;

            // 目標色に最も近いチューブを選ぶ
            for i in 0..k {
                let dist = euclidean_distance(&tubes[i].color, target_color);
                if dist < min_distance {
                    min_distance = dist;
                    selected_tube_index = i;
                }
            }

            // 操作1: 近いチューブから絵の具を追加
            let i = rng.gen_range(0..n);
            let j = rng.gen_range(0..n);
            println!("1 {} {} {}", i, j, selected_tube_index);
            operation_count += 1;
        }

        // 操作2: 目標色を作るために1グラム絵の具を取り出して画伯に渡す
        if total_operations < h {
            let i = rng.gen_range(0..n);
            let j = rng.gen_range(0..n);
            println!("2 {} {}", i, j);
            total_operations += 1;
        }

        operation_count += 1;
    }

    io::stdout().flush().unwrap();
}

// 2309646
