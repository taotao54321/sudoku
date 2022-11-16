use crate::square::Square;

/// マスの集合を表す bitboard。
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct Bitboard(u128);

impl Bitboard {
    /// 全マスが 0 の bitboard を返す。
    pub(crate) fn empty() -> Self {
        Self(0)
    }

    /// 全マスが 1 の bitboard を返す。
    pub(crate) fn full() -> Self {
        Self((1 << Square::NUM) - 1)
    }

    /// 全マスが 0 かどうかを返す。
    pub(crate) fn is_zero(self) -> bool {
        self.0 == 0
    }

    /// 指定したマスを 1 にする。
    pub(crate) fn add(&mut self, sq: Square) {
        self.0 |= 1 << sq.get();
    }

    /// 指定したマスを 0 にする。
    pub(crate) fn remove(&mut self, sq: Square) {
        self.0 &= !(1 << sq.get());
    }

    /// 内部値最小の 1 のマスを 0 に変え、そのマスを返す。
    pub(crate) fn pop(&mut self) -> Option<Square> {
        (self.0 != 0).then(|| {
            let i = self.0.trailing_zeros();
            self.0 &= !(1 << i);
            unsafe { Square::new_unchecked(i as u8) }
        })
    }

    /// 1 のマスを昇順で列挙する。
    pub(crate) fn iter(mut self) -> impl Iterator<Item = Square> + std::iter::FusedIterator {
        std::iter::from_fn(move || self.pop()).fuse()
    }
}
