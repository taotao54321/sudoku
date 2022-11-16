use rand::prelude::*;

use crate::bitboard::Bitboard;
use crate::board::Board;
use crate::number::Number;
use crate::square::*;
use crate::used_mask::UsedMasks;

/// 数独の局面。
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Sudoku {
    board: Board,
    used_masks: UsedMasks,
}

impl Sudoku {
    /// 盤面を指定して局面を作る。
    pub fn new(board: Board) -> Self {
        let mut used_masks = UsedMasks::all_unused();
        for sq in Square::all() {
            if let Some(num) = board[sq] {
                used_masks.use_number(sq, num);
            }
        }

        Self { board, used_masks }
    }

    /// 盤面を返す。
    pub fn board(&self) -> &Board {
        &self.board
    }

    /// 局面が既に解けているかどうかを返す。
    pub fn is_solved(&self) -> bool {
        self.board.is_solved()
    }

    /// マス `sq` に数 `num` を置くことを試みる。実際に置けたかどうかを返す。
    pub fn put(&mut self, sq: Square, num: Number) -> bool {
        if !self.used_masks.is_legal(sq, num) {
            return false;
        }
        self.put_number(sq, num);
        true
    }

    /// 局面を解くことを試み、解けたかどうかを返す。
    ///
    /// 結果によらず、盤面は埋められるだけ埋められる。
    pub fn solve(&mut self) -> bool {
        let bb_vacant = calc_bb_vacant(&self.board);
        self.solve_impl(bb_vacant)
    }

    fn solve_impl(&mut self, mut bb_vacant: Bitboard) -> bool {
        // 次の空きマスを得る。空きマスがなければ解けている。
        let Some(sq) = self.pop_best_vacant_square(&mut bb_vacant) else {
            return true;
        };

        // 候補を順に試す。
        for num in self.used_masks.candidates(sq) {
            self.put_number(sq, num);
            if self.solve_impl(bb_vacant) {
                return true;
            }
            self.remove_number(sq, num);
        }

        // どの候補もダメなら解けない。
        false
    }

    /// 局面が解けるかどうかを返す。一意性は問わない。
    ///
    /// 呼び出し前後で局面は変化しない。
    pub fn is_solvable(&mut self) -> bool {
        let bb_vacant = calc_bb_vacant(&self.board);
        self.is_solvable_impl(bb_vacant)
    }

    fn is_solvable_impl(&mut self, mut bb_vacant: Bitboard) -> bool {
        let Some(sq) = self.pop_best_vacant_square(&mut bb_vacant) else {
            return true;
        };

        for num in self.used_masks.candidates(sq) {
            self.put_number(sq, num);
            let ok = self.is_solvable_impl(bb_vacant);
            self.remove_number(sq, num);
            if ok {
                return true;
            }
        }

        false
    }

    /// 局面が一意に解けるかどうかを返す。
    ///
    /// 呼び出し前後で局面は変化しない。
    pub fn is_unique_solvable(&mut self) -> bool {
        let bb_vacant = calc_bb_vacant(&self.board);
        self.is_unique_solvable_impl(bb_vacant)
    }

    fn is_unique_solvable_impl(&mut self, bb_vacant: Bitboard) -> bool {
        #[derive(Debug)]
        struct Search<'a> {
            sudoku: &'a mut Sudoku,
            count: u32,
        }
        impl<'a> Search<'a> {
            fn new(sudoku: &'a mut Sudoku) -> Self {
                Self { sudoku, count: 0 }
            }
            fn search(&mut self, mut bb_vacant: Bitboard) {
                let Some(sq) = self.sudoku.pop_best_vacant_square(&mut bb_vacant) else {
                    self.count += 1;
                    return;
                };
                for num in self.sudoku.used_masks.candidates(sq) {
                    self.sudoku.put_number(sq, num);
                    self.search(bb_vacant);
                    self.sudoku.remove_number(sq, num);
                    if self.count >= 2 {
                        return;
                    }
                }
            }
        }

        let mut search = Search::new(self);
        search.search(bb_vacant);

        search.count == 1
    }

    /// 一意に解ける局面をランダムに生成し、その局面および解を返す。
    /// `hint_min` は最小ヒント数。
    pub fn generate_unique(hint_min: u32) -> (Self, Self) {
        // 既に解けているランダムな局面から開始する。これは自明に一意解をもつ。
        // 局面から数をランダムな順序で消していく。
        // そのマスにおいて消した数の他に選択肢がなければ、消した後の局面もまた一意解をもつ。
        // 他に選択肢がある場合、他の解があるとすればそのマスに違う数が入ったものとなる(ほぼ自明。背理法で示せる)。
        // 全てのマスを試し終わったら終了。

        let mut rng = thread_rng();
        let mut sudoku = Self::generate_solved(&mut rng);
        let solution = sudoku.clone();
        let mut hint = 81;

        let mut sqs = Square::all();
        sqs.shuffle(&mut rng);
        for sq in sqs {
            if hint <= hint_min {
                break;
            }

            // 数を消す。
            let num = sudoku.board[sq].unwrap();
            sudoku.remove_number(sq, num);

            // 他の解を探す。
            let mut found = false;
            for num_other in sudoku.used_masks.candidates(sq).filter(|&e| e != num) {
                sudoku.put_number(sq, num_other);
                found = sudoku.is_solvable();
                sudoku.remove_number(sq, num_other);
                if found {
                    break;
                }
            }

            // 他の解がなければ消したままでよい。
            // 他の解がある場合、消してはいけないので sq に num を置き直す。
            if found {
                sudoku.put_number(sq, num);
            } else {
                hint -= 1;
            }
        }

        (sudoku, solution)
    }

    /// 既に解けている局面をランダムに生成する。
    fn generate_solved<R>(rng: &mut R) -> Self
    where
        R: Rng,
    {
        // 空の局面から開始し、ランダムなマスにランダムな数を置く。
        // その局面に解がなければ別の置き方を試す。
        // 解があれば続けてランダムに数を置く。これを繰り返す。
        //
        // 「空の局面から開始し、マス/数ともにランダムな列挙順で解く」のはうまくいかない。
        // 解く際は is_solvable() を使わないと時間がかかりすぎるようだ。
        //
        // NOTE: is_solvable() を使ってもまれに返ってこなくなることがある。
        // 対策として is_solvable() をノード数制限付きで行い、制限にかかったら最初からやり直す。

        loop {
            if let Some(sudoku) = Self::try_generate_solved(rng) {
                return sudoku;
            }
        }
    }

    fn try_generate_solved<R>(rng: &mut R) -> Option<Self>
    where
        R: Rng,
    {
        let mut sudoku = Sudoku::new(Board::empty());
        let mut bb_vacant = Bitboard::full();

        while !bb_vacant.is_zero() {
            let sq = bb_vacant.iter().choose(rng).unwrap();
            let num = sudoku.used_masks.candidates(sq).choose(rng).unwrap();

            sudoku.put_number(sq, num);
            bb_vacant.remove(sq);
            let Some(ok) = sudoku.is_solvable_limited(10_u32.pow(6)) else {
                return None;
            };
            if !ok {
                sudoku.remove_number(sq, num);
                bb_vacant.add(sq);
            }
        }

        Some(sudoku)
    }

    /// ノード数制限付きの `is_solvable()`。
    /// 探索中にノード数制限に達した場合、`None` を返す。
    ///
    /// 呼び出し前後で局面は変化しない。
    fn is_solvable_limited(&mut self, node_count_max: u32) -> Option<bool> {
        #[derive(Debug)]
        struct Search<'a> {
            sudoku: &'a mut Sudoku,
            node_count_max: u32,
            node_count: u32,
        }
        impl<'a> Search<'a> {
            fn new(sudoku: &'a mut Sudoku, node_count_max: u32) -> Self {
                Self {
                    sudoku,
                    node_count_max,
                    node_count: 0,
                }
            }
            fn search(&mut self) -> Option<bool> {
                let bb_vacant = calc_bb_vacant(&self.sudoku.board);
                self.search_impl(bb_vacant)
            }
            fn search_impl(&mut self, mut bb_vacant: Bitboard) -> Option<bool> {
                self.node_count += 1;
                if self.node_count >= self.node_count_max {
                    return None;
                }
                let Some(sq) = self.sudoku.pop_best_vacant_square(&mut bb_vacant) else {
                    return Some(true);
                };
                for num in self.sudoku.used_masks.candidates(sq) {
                    self.sudoku.put_number(sq, num);
                    let ok = self.search_impl(bb_vacant);
                    self.sudoku.remove_number(sq, num);
                    // 解けたorノード数制限に達したら打ち切る。
                    if ok != Some(false) {
                        return ok;
                    }
                }
                Some(false)
            }
        }

        let mut search = Search::new(self, node_count_max);
        search.search()
    }

    fn put_number(&mut self, sq: Square, num: Number) {
        self.board[sq] = Some(num);
        self.used_masks.use_number(sq, num);
    }

    fn remove_number(&mut self, sq: Square, num: Number) {
        self.board[sq] = None;
        self.used_masks.unuse_number(sq, num);
    }

    /// 空きマス集合 `bb_vacant` から数の候補が最も少ないものを pop する。
    fn pop_best_vacant_square(&self, bb_vacant: &mut Bitboard) -> Option<Square> {
        let sq = bb_vacant
            .iter()
            .min_by_key(|&sq| self.used_masks.candidate_count(sq));
        if let Some(sq) = sq {
            bb_vacant.remove(sq);
        }
        sq
    }
}

/// 盤面の空きマスを表す bitboard を求める。
fn calc_bb_vacant(board: &Board) -> Bitboard {
    let mut bb_vacant = Bitboard::empty();
    for sq in Square::all().into_iter().filter(|&sq| board[sq].is_none()) {
        bb_vacant.add(sq);
    }
    bb_vacant
}
