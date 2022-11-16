use crate::number::Number;
use crate::square::*;

/// 数の使用状況を管理する。
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct UsedMasks {
    col_mask: UsedMask,
    row_mask: UsedMask,
    block_mask: UsedMask,
}

impl UsedMasks {
    /// 解き終えた盤面に対応する `UsedMasks` を返す。
    #[allow(dead_code)]
    pub(crate) fn all_used() -> Self {
        Self {
            col_mask: UsedMask::all_used(),
            row_mask: UsedMask::all_used(),
            block_mask: UsedMask::all_used(),
        }
    }

    /// 空の盤面に対応する `UsedMasks` を返す。
    pub(crate) fn all_unused() -> Self {
        Self {
            col_mask: UsedMask::all_unused(),
            row_mask: UsedMask::all_unused(),
            block_mask: UsedMask::all_unused(),
        }
    }

    /// 空きマス `sq` に数 `num` を書いたとして状態を更新する。
    pub(crate) fn use_number(&mut self, sq: Square, num: Number) {
        self.col_mask.use_number(sq.col().get(), num);
        self.row_mask.use_number(sq.row().get(), num);
        self.block_mask.use_number(sq.block().get(), num);
    }

    /// マス `sq` に書かれた数 `num` を消したとして状態を更新する。
    pub(crate) fn unuse_number(&mut self, sq: Square, num: Number) {
        self.col_mask.unuse_number(sq.col().get(), num);
        self.row_mask.unuse_number(sq.row().get(), num);
        self.block_mask.unuse_number(sq.block().get(), num);
    }

    /// マス `sq` に数 `num` を置く手が合法かどうかを返す。
    pub(crate) fn is_legal(&self, sq: Square, num: Number) -> bool {
        let mask = self.candidate_mask(sq);
        (mask & (1 << (num.get() - 1))) != 0
    }

    /// マス `sq` に入りうる数を昇順で列挙する。
    pub(crate) fn candidates(
        &self,
        sq: Square,
    ) -> impl Iterator<Item = Number> + std::iter::FusedIterator {
        let mut mask = self.candidate_mask(sq);
        std::iter::from_fn(move || {
            (mask != 0).then(|| {
                let i = mask.trailing_zeros();
                mask &= !(1 << i);
                unsafe { Number::new_unchecked((i + 1) as u8) }
            })
        })
        .fuse()
    }

    /// マス `sq` に入りうる数の個数を返す。
    pub(crate) fn candidate_count(&self, sq: Square) -> u32 {
        self.candidate_mask(sq).count_ones()
    }

    fn candidate_mask(&self, sq: Square) -> u32 {
        const MASK_NUMS: u32 = (1 << 9) - 1;

        let col_mask = (self.col_mask.0 >> (9 * sq.col().get())) as u32;
        let row_mask = (self.row_mask.0 >> (9 * sq.row().get())) as u32;
        let block_mask = (self.block_mask.0 >> (9 * sq.block().get())) as u32;

        col_mask & row_mask & block_mask & MASK_NUMS
    }
}

/// 列or行orブロックに関する使用済みの数のマスク。
///
/// 内部的には未使用の数を 1 とする。
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct UsedMask(u128);

impl UsedMask {
    /// 解き終えた盤面に対応するマスクを返す。
    fn all_used() -> Self {
        Self(0)
    }

    /// 空の盤面に対応するマスクを返す。
    fn all_unused() -> Self {
        Self((1 << 81) - 1)
    }

    /// `i` 番目の列or行orブロックについて数 `num` を使用済みとする。
    fn use_number(&mut self, i: u8, num: Number) {
        self.0 &= !(1 << (9 * i + num.get() - 1));
    }

    /// `i` 番目の列or行orブロックについて数 `num` を未使用とする。
    fn unuse_number(&mut self, i: u8, num: Number) {
        self.0 |= 1 << (9 * i + num.get() - 1);
    }
}
