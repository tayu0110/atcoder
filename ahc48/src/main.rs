use rand::seq::SliceRandom; // shuffleのために追加
use rand::Rng;
use std::collections::{HashMap, HashSet};
use std::f64::INFINITY;
use std::io::{self, Write};

// --- 定数定義 ---
const N_PALETTE: usize = 20; // Nは常に20と仮定
const THRESHOLD_NEAR_ONE: f64 = 1.0 - 1e-6; // 1グラムとみなす閾値 (絵の具を取り出す最小量)
const MIN_MIX_INCREMENT_VOLUME: f64 = 0.01; // 1回の混合操作で追加する絵の具の最小量 (グラム)

// 各ターゲットを達成するために最低限必要なターン数 (廃棄を含む/含まない)
const MIN_TURNS_PER_TARGET_WITH_CLEAR: usize = 4; // 廃棄(1) + 混合2回(2) + 取り出し(1)
const MIN_TURNS_PER_TARGET_WITHOUT_CLEAR: usize = 3; // 混合2回(2) + 取り出し(1)
const MAX_COLOR_ERROR_FOR_REUSE: f64 = 1.0; // 誤差1未満を実現できるウェルを再利用する際の誤差上限

// --- 構造体定義 ---
#[derive(Debug, Clone, Copy, PartialEq)]
struct Paint {
    c: f64,
    m: f64,
    y: f64,
}

impl Paint {
    // 絵の具の初期化
    fn new(c: f64, m: f64, y: f64) -> Self {
        Paint { c, m, y }
    }

    // 2つの色のユークリッド距離
    fn euclidean_distance(&self, other: &Paint) -> f64 {
        let dc = self.c - other.c;
        let dm = self.m - other.m;
        let dy = self.y - other.y;
        (dc * dc + dm * dm + dy * dy).sqrt()
    }

    // 色の線形補間（絵の具の混合）
    // v1, v2 はそれぞれの絵の具の量
    fn mix(p1: &Paint, v1: f64, p2: &Paint, v2: f64) -> Self {
        let total_volume = v1 + v2;
        if total_volume <= 1e-9 {
            // ほぼ0の場合（浮動小数点誤差を考慮）
            return Paint::new(0.0, 0.0, 0.0); // 絵の具がない場合
        }
        Paint::new(
            (p1.c * v1 + p2.c * v2) / total_volume,
            (p1.m * v1 + p2.m * v2) / total_volume,
            (p1.y * v1 + p2.y * v2) / total_volume,
        )
    }

    // 2色混合で目標色に最も近づく比率を計算する (クランプあり)
    fn get_best_mix_ratio_2colors(target_color: &Paint, p1: &Paint, p2: &Paint) -> (f64, f64) {
        let v1_to_target = Paint::new(
            target_color.c - p1.c,
            target_color.m - p1.m,
            target_color.y - p1.y,
        );
        let v1_to_v2 = Paint::new(p2.c - p1.c, p2.m - p1.m, p2.y - p1.y);

        let dot_product =
            v1_to_target.c * v1_to_v2.c + v1_to_target.m * v1_to_v2.m + v1_to_target.y * v1_to_v2.y;

        let v1_to_v2_len_sq =
            v1_to_v2.c * v1_to_v2.c + v1_to_v2.m * v1_to_v2.m + v1_to_v2.y * v1_to_v2.y;

        if v1_to_v2_len_sq <= 1e-9 {
            // p1とp2がほぼ同じ色の場合
            return (1.0, 0.0); // p1のみ
        }

        let mut alpha = dot_product / v1_to_v2_len_sq;

        // alphaを0から1の範囲にクランプする (線分P1P2の外側にはみ出さないように)
        alpha = alpha.max(0.0).min(1.0);

        (1.0 - alpha, alpha) // (p1の比率, p2の比率)
    }
}

// チューブ絵の具
#[derive(Debug, Clone)]
struct Tube {
    color: Paint,
}

// 目標色
#[derive(Debug, Clone)]
struct Target {
    color: Paint,
}

// ウェル内の状態
#[derive(Debug, Clone)]
struct Well {
    current_color: Paint,
    current_volume: f64,
    capacity: usize, // マスの数 = 最大容量 (今回は常に1)
}

impl Well {
    fn new() -> Self {
        Well {
            current_color: Paint::new(0.0, 0.0, 0.0), // 最初は透明
            current_volume: 0.0,
            capacity: 1, // 各マスは容量1
        }
    }
    // 指定した色と量で新しいウェルを作成
    fn new_with_data(color: Paint, volume: f64, capacity: usize) -> Self {
        Well {
            current_color: color,
            current_volume: volume,
            capacity,
        }
    }

    // 絵の具を追加
    // tube_volume はチューブから出る量（通常1グラム）
    // 実際に混ざった量を返す
    fn add_paint(&mut self, tube_color: &Paint, tube_volume: f64) -> f64 {
        let remaining_capacity = self.capacity as f64 - self.current_volume;
        let actual_add_volume = tube_volume.min(remaining_capacity).max(0.0);

        if actual_add_volume <= 1e-9 {
            return 0.0;
        }

        if self.current_volume <= 1e-9 {
            self.current_color = tube_color.clone();
            self.current_volume = actual_add_volume;
        } else {
            self.current_color = Paint::mix(
                &self.current_color,
                self.current_volume,
                tube_color,
                actual_add_volume,
            );
            self.current_volume += actual_add_volume;
        }
        actual_add_volume
    }

    // 絵の具を取り出す
    // 実際に取り出した量を返す
    fn take_paint(&mut self) -> f64 {
        if self.current_volume < THRESHOLD_NEAR_ONE {
            return 0.0; // 1グラム未満と見なされ、操作2は実行できない
        }

        let taken_volume = if self.current_volume >= 1.0 {
            1.0 // 1グラム以上あればちょうど1グラム取り出す
        } else {
            self.current_volume // 閾値以上1グラム未満なら全て取り出す
        };
        self.current_volume -= taken_volume;

        if self.current_volume <= 1e-9 {
            self.current_color = Paint::new(0.0, 0.0, 0.0);
        }
        taken_volume
    }

    // 絵の具を廃棄
    // 実際に廃棄した量を返す
    fn discard_paint(&mut self) -> f64 {
        let discarded_volume = self.current_volume;
        self.current_volume = 0.0;
        self.current_color = Paint::new(0.0, 0.0, 0.0);
        discarded_volume
    }
}

// Union-Find (Disjoint Set Union) for managing wells - 今回は結合しないので実質使わないが構造として保持
struct UnionFind {
    parent: Vec<usize>,
    well_data: HashMap<usize, Well>,
    n_grid: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let n_grid = n * n;
        let mut parent = vec![0; n_grid];
        let mut well_data = HashMap::new();
        for i in 0..n_grid {
            parent[i] = i;
            well_data.insert(i, Well::new_with_data(Paint::new(0.0, 0.0, 0.0), 0.0, 1));
        }
        UnionFind {
            parent,
            well_data,
            n_grid,
        }
    }

    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] == i {
            i
        } else {
            let root = self.find(self.parent[i]);
            self.parent[i] = root;
            root
        }
    }

    // Union操作は今回は行わないため、実装は元のままで結合はされない前提
    fn union(&mut self, i: usize, j: usize) -> bool {
        let root_i = self.find(i);
        let root_j = self.find(j);

        if root_i != root_j {
            let size_i = self.well_data.get(&root_i).map_or(0, |w| w.capacity);
            let size_j = self.well_data.get(&root_j).map_or(0, |w| w.capacity);

            let (small_root, large_root) = if size_i < size_j {
                (root_i, root_j)
            } else {
                (root_j, root_i)
            };

            self.parent[small_root] = large_root;

            let small_well = self.well_data.remove(&small_root).unwrap_or_else(Well::new);
            let mut large_well = self.well_data.remove(&large_root).unwrap_or_else(Well::new);

            large_well.current_color = Paint::mix(
                &large_well.current_color,
                large_well.current_volume,
                &small_well.current_color,
                small_well.current_volume,
            );
            large_well.current_volume += small_well.current_volume;
            large_well.capacity += small_well.capacity;

            self.well_data.insert(large_root, large_well);
            true
        } else {
            false
        }
    }
}

// パレット全体の状態を管理する構造体
struct Pallet {
    n: usize,
    uf: UnionFind,
    vertical_bars: Vec<Vec<i32>>,
    horizontal_bars: Vec<Vec<i32>>,
    coord_to_idx: Box<dyn Fn(usize, usize) -> usize>,
    idx_to_coord: Box<dyn Fn(usize) -> (usize, usize)>,
    // 追加: 使用済みのマスを追跡
    used_cells: HashSet<(usize, usize)>,
}

impl Pallet {
    fn new(n: usize) -> Self {
        let coord_to_idx_fn = move |r: usize, c: usize| r * n + c;
        let idx_to_coord_fn = move |idx: usize| (idx / n, idx % n);

        Pallet {
            n,
            uf: UnionFind::new(n),
            vertical_bars: vec![vec![1; n - 1]; n],
            horizontal_bars: vec![vec![1; n]; n - 1],
            coord_to_idx: Box::new(coord_to_idx_fn),
            idx_to_coord: Box::new(idx_to_coord_fn),
            used_cells: HashSet::new(), // 初期化
        }
    }

    // 今回は仕切り操作を行わないため、このメソッドは事実上使用しないが、構造として保持
    fn operate_bar(&mut self, _r1: usize, _c1: usize, _r2: usize, _c2: usize, _bar_type: i32) {
        // この戦略では仕切り操作は行わないため、何もしない
    }

    // 指定されたマスが属するウェルの情報を取得 (mutable)
    fn get_well_info(&mut self, r: usize, c: usize) -> Option<&mut Well> {
        let idx = (self.coord_to_idx)(r, c);
        let root = self.uf.find(idx);
        self.uf.well_data.get_mut(&root)
    }

    // 指定されたマスが属するウェルの情報を immutable で取得
    fn get_well_info_immutable(&self, r: usize, c: usize) -> Option<&Well> {
        let idx = (self.coord_to_idx)(r, c);
        // find は mutable なので、immutable な参照のために直接 parent を辿る
        let mut current = idx;
        while self.uf.parent[current] != current {
            current = self.uf.parent[current];
        }
        let root = current;
        self.uf.well_data.get(&root)
    }

    // 指定されたウェルIDのウェルを空にする（絵の具を廃棄）
    fn clear_well(&mut self, r: usize, c: usize) {
        if let Some(well) = self.get_well_info(r, c) {
            well.discard_paint();
        }
    }

    // 未使用のマスを取得する
    fn get_unused_cells(&self) -> Vec<(usize, usize)> {
        let mut unused = Vec::new();
        for r in 0..self.n {
            for c in 0..self.n {
                if !self.used_cells.contains(&(r, c)) {
                    unused.push((r, c));
                }
            }
        }
        unused
    }
}

// --- 入力ヘルパー関数 ---
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

// --- メイン処理 ---
fn main() {
    let params: Vec<usize> = read_usize_line();
    let n = params[0]; // N_PALETTE
    let k = params[1]; // N_TUBE
    let h = params[2]; // N_TARGET
    let t = params[3]; // MAX_TURN
    let _d = params[4]; // MAX_DISTANCE_THRESHOLD (スコア計算用、コードでは直接使わない)

    let mut tubes = Vec::new();
    for _u in 0..k {
        let colors: Vec<f64> = read_f64_line();
        tubes.push(Tube {
            color: Paint::new(colors[0], colors[1], colors[2]),
        });
    }

    let mut targets = Vec::new();
    for _u in 0..h {
        let colors: Vec<f64> = read_f64_line();
        targets.push(Target {
            color: Paint::new(colors[0], colors[1], colors[2]),
        });
    }

    // パレットの初期状態を生成（全て仕切りが出ている状態）
    let mut pallet = Pallet::new(n);

    // 初期仕切り配置の出力 (全て1: 仕切りが出ている状態)
    for i in 0..n {
        for j in 0..(n - 1) {
            print!("1 "); // 縦の仕切り
        }
        println!();
    }
    for i in 0..(n - 1) {
        for j in 0..n {
            print!("1 "); // 横の仕切り
        }
        println!();
    }

    // --- メインループ：H回のターゲット達成を目指す ---
    let mut current_turn = 0;
    let mut targets_made_count = 0; // 実際に操作2を行った回数

    let mut rng = rand::thread_rng();

    // H個のターゲットを全て作成するまでループを続ける
    // ただし、総ターン数 T を超えない範囲で
    while targets_made_count < h && current_turn < t {
        let target_color = &targets[targets_made_count].color;

        let remaining_targets = h - targets_made_count;
        let remaining_turns = t.saturating_sub(current_turn);

        let mut r_mix: usize = 0;
        let mut c_mix: usize = 0;
        let mut clear_current_well = false; // 今回のターゲットでウェルをクリアするかどうか

        // --- ウェル選択ロジック ---
        // 1. 未使用のウェルを優先
        let mut unused_cells = pallet.get_unused_cells();
        if let Some((ur, uc)) = unused_cells.pop() {
            // 未使用のマスがあれば1つ取り出す
            r_mix = ur;
            c_mix = uc;
            pallet.used_cells.insert((r_mix, c_mix)); // 使用済みにマーク
            clear_current_well = false; // 未使用なのでクリア不要
        } else {
            // 2. 未使用のウェルがない場合、誤差1未満を実現できるウェルがあれば再利用
            let mut best_reuse_cell: Option<(usize, usize, f64)> = None; // (r, c, distance)

            for r_iter in 0..n {
                for c_iter in 0..n {
                    if let Some(well) = pallet.get_well_info_immutable(r_iter, c_iter) {
                        if well.current_volume > 1e-9 {
                            // 絵の具があるウェルのみ
                            let dist = well.current_color.euclidean_distance(target_color);
                            if dist < MAX_COLOR_ERROR_FOR_REUSE {
                                // 誤差が許容範囲内
                                if best_reuse_cell.is_none() || dist < best_reuse_cell.unwrap().2 {
                                    best_reuse_cell = Some((r_iter, c_iter, dist));
                                }
                            }
                        }
                    }
                }
            }

            if let Some((br, bc, _)) = best_reuse_cell {
                r_mix = br;
                c_mix = bc;
                clear_current_well = false; // 再利用なのでクリア不要
            } else {
                // 3. 上記に該当するウェルがない場合、ランダムにウェルを1つだけクリアし、利用
                // すでに使われたことがあるマスの中からランダムに選択
                let mut all_cells: Vec<(usize, usize)> = Vec::new();
                for r_iter in 0..n {
                    for c_iter in 0..n {
                        all_cells.push((r_iter, c_iter));
                    }
                }
                // 使用されているマスの中からランダムに選ぶ（完全にランダムにすると未利用マスもクリアしてしまう）
                let potential_clear_cells: Vec<(usize, usize)> = all_cells
                    .iter()
                    .filter(|&coord| pallet.used_cells.contains(coord))
                    .cloned()
                    .collect();

                let (rand_r, rand_c) = if !potential_clear_cells.is_empty() {
                    *potential_clear_cells.choose(&mut rng).unwrap()
                } else {
                    // 全マスが未使用の場合（非常に稀だが念のため）、ランダムな1マスを選択
                    (rng.gen_range(0..n), rng.gen_range(0..n))
                };

                r_mix = rand_r;
                c_mix = rand_c;
                pallet.used_cells.insert((r_mix, c_mix)); // 使用済みにマーク
                clear_current_well = true; // このウェルをクリアする必要がある
            }
        }

        // --- ウェルをクリア (操作3) の判断と実行 ---
        // clear_current_well が true かつ、ターンに余裕がない場合はスキップするロジックも継続
        if clear_current_well {
            // ウェルに絵の具が残っている場合のみ考慮
            if let Some(well) = pallet.get_well_info_immutable(r_mix, c_mix) {
                if well.current_volume > 1e-9 {
                    // クリア操作を含めた場合の残りターン
                    let turns_needed_with_clear =
                        remaining_targets * MIN_TURNS_PER_TARGET_WITH_CLEAR;
                    // クリア操作をスキップした場合の残りターン
                    let turns_needed_without_clear =
                        remaining_targets * MIN_TURNS_PER_TARGET_WITHOUT_CLEAR;

                    if turns_needed_with_clear > remaining_turns
                        && turns_needed_without_clear <= remaining_turns
                    {
                        // クリアしたら間に合わないが、クリアしなければ間に合う可能性があれば、
                        // clear_current_wellをfalseに上書きしてスキップ
                        clear_current_well = false;
                    }
                } else {
                    // ウェルがすでに空ならクリア操作は不要
                    clear_current_well = false;
                }
            } else {
                // get_well_info_immutableがNoneを返すことはUnionFindの初期化が正しければありえないが念のため
                clear_current_well = false;
            }
        }

        if clear_current_well {
            println!("3 {} {}", r_mix, c_mix);
            current_turn += 1;
            if current_turn >= t {
                break;
            } // ターン切れ
            pallet.clear_well(r_mix, c_mix); // 内部状態もクリア
        }
        // else: clear_current_wellがfalseの場合は、ウェルの内容をそのまま使用する

        // --- 2. 最適な2つのチューブを選択 ---
        let mut tube_distances: Vec<(f64, usize)> = Vec::new(); // (距離, チューブインデックス)
        for i in 0..k {
            let dist = tubes[i].color.euclidean_distance(target_color);
            tube_distances.push((dist, i));
        }
        tube_distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        let tube1_idx = tube_distances[0].1;
        let tube1_color = &tubes[tube1_idx].color;

        let tube2_idx = if k > 1 {
            tube_distances[1].1 // 2番目に近いチューブ
        } else {
            tube_distances[0].1 // チューブが1種類しかない場合は同じチューブを使う (混合は単色になる)
        };
        let tube2_color = &tubes[tube2_idx].color;

        // --- 3. 絵の具の混合 ---
        let target_volume: f64 = 1.0; // 目標は1グラムの絵の具
        let mut mix_attempts_for_current_target = 0;

        // 残りターゲット数と残りターン数から、このターゲットに許容される混合操作のターン数を概算
        let turns_available_for_mixing = if remaining_targets > 0 {
            // 廃棄をスキップした場合は3ターン、廃棄した場合は4ターン消費するとして計算
            let base_turns_per_target = if clear_current_well {
                MIN_TURNS_PER_TARGET_WITH_CLEAR
            } else {
                MIN_TURNS_PER_TARGET_WITHOUT_CLEAR
            };

            // 残りターンから、残りのターゲット数 * ベースターン数を引いて、残りの混合可能ターンを計算
            let calculated_turns = (remaining_turns as isize
                - (remaining_targets as isize * base_turns_per_target as isize))
                .max(0) as usize;
            // 計算されたターンを残りターゲット数で割ることで、このターゲットに使える混合ターン数を算出
            calculated_turns / remaining_targets
        } else {
            1000 // H個全て達成済みの場合はここには来ないが、念のため適当な上限
        };
        // 少なくとも1回は混合を試みる (2回の操作1で構成されるので、実際には2ターン消費)
        let max_mix_turns_for_this_target = turns_available_for_mixing.max(1);

        loop {
            if current_turn >= t {
                break;
            } // ターン切れ
            if mix_attempts_for_current_target * 2 >= max_mix_turns_for_this_target {
                break;
            } // 混合操作回数上限

            let current_well_volume = pallet
                .get_well_info_immutable(r_mix, c_mix)
                .unwrap()
                .current_volume;
            let remaining_volume_to_add = target_volume - current_well_volume;

            if remaining_volume_to_add <= 1e-9 && current_well_volume >= THRESHOLD_NEAR_ONE {
                // すでに目標量に達しているか、それ以上ある場合は混合を終了
                break;
            }

            // 追加する絵の具の量
            // 残りの必要量と、最小混合量を考慮
            let mut add_volume_per_tube =
                (remaining_volume_to_add / 2.0).max(MIN_MIX_INCREMENT_VOLUME);
            // ターン数を考慮し、残りのターンで目標量に到達できるように調整
            if max_mix_turns_for_this_target > 0 {
                let turns_left_for_mixing = (max_mix_turns_for_this_target as f64
                    - (mix_attempts_for_current_target * 2) as f64)
                    .max(1.0);
                add_volume_per_tube =
                    add_volume_per_tube.max(remaining_volume_to_add / turns_left_for_mixing);
            }

            // 混合比率を計算
            let (p1_ratio, p2_ratio) =
                Paint::get_best_mix_ratio_2colors(target_color, tube1_color, tube2_color);

            // 実際に混ぜる量
            let vol_to_add_t1 = (add_volume_per_tube * p1_ratio).min(remaining_volume_to_add);
            let vol_to_add_t2 =
                (add_volume_per_tube * p2_ratio).min(remaining_volume_to_add - vol_to_add_t1);

            // 操作1: チューブ1から絵の具を追加
            if vol_to_add_t1 > 1e-9 {
                // vol_to_add_t1 の typo を修正
                println!("1 {} {} {}", r_mix, c_mix, tube1_idx);
                if let Some(well) = pallet.get_well_info(r_mix, c_mix) {
                    well.add_paint(tube1_color, vol_to_add_t1);
                }
                current_turn += 1;
                if current_turn >= t {
                    break;
                }
            }

            // 操作1: チューブ2から絵の具を追加
            if vol_to_add_t2 > 1e-9 {
                println!("1 {} {} {}", r_mix, c_mix, tube2_idx);
                if let Some(well) = pallet.get_well_info(r_mix, c_mix) {
                    well.add_paint(tube2_color, vol_to_add_t2);
                }
                current_turn += 1;
                if current_turn >= t {
                    break;
                }
            }

            mix_attempts_for_current_target += 1;
        }

        // --- 4. 絵の具の取り出し (操作2) ---
        // ここで必ず操作2を試みる。必要な量に達していなければ、スコアは無限大になる。
        // 目標量が確保できていれば、必ず操作2を実行する。
        if current_turn < t {
            if let Some(well) = pallet.get_well_info(r_mix, c_mix) {
                let taken_volume = well.take_paint();
                if taken_volume >= THRESHOLD_NEAR_ONE {
                    // 実際に1グラム以上取り出せたか
                    println!("2 {} {}", r_mix, c_mix);
                    targets_made_count += 1;
                    current_turn += 1;
                } else {
                    // 1グラム以上取り出せなかった場合 (このケースは極力避けるべき)
                    // H回達成の制約を破ることになるため、スコアは無限大になる。
                    // しかし、ここで無限ループに陥るのを避けるため、次のターゲットへ進む。
                    // この問題の根本的な解決は、混合戦略の精度向上か、Tの制潤緩和。
                    // ロギングやデバッグでこのelseブロックに入っているか確認すると良い。
                }
            }
        }
    }

    // --- 残りのターンをダミー操作で埋める ---
    // H回の操作2が完了していても、Tターンに達していなければ、残りのターンを消費する
    while current_turn < t {
        // ランダムなマスを選んで廃棄するのではなく、最後に利用したマスを廃棄する
        // または、特定のマス(例えば(0,0))を廃棄する
        // ここでは、シンプルに(0,0)を廃棄することにする。
        // 未使用ウェル活用により、多くのマスにゴミが残る可能性もあるが、最後のターン埋めはスコアに影響しない。
        println!("3 {} {}", 0, 0);
        current_turn += 1;
    }

    io::stdout().flush().unwrap();
}
