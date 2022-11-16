use std::num::NonZeroU8;

use crate::macros::assert_unchecked;

/// マスに書かれる数。
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Number(NonZeroU8);

impl Number {
    pub const MIN_VALUE: u8 = 1;
    pub const MAX_VALUE: u8 = 9;

    /// マスに書かれる数の総数。
    pub const NUM: usize = 9;

    /// 内部値を指定して数を作る。
    pub fn new(inner: u8) -> Option<Self> {
        matches!(inner, Self::MIN_VALUE..=Self::MAX_VALUE)
            .then(|| unsafe { Self::new_unchecked(inner) })
    }

    /// 内部値を指定して数を作る。
    ///
    /// # Safety
    ///
    /// `inner` は有効値でなければならない。
    pub unsafe fn new_unchecked(inner: u8) -> Self {
        assert_unchecked!(matches!(inner, Self::MIN_VALUE..=Self::MAX_VALUE));
        Self(NonZeroU8::new_unchecked(inner))
    }

    /// 内部値を返す。
    pub fn get(self) -> u8 {
        self.0.get()
    }

    /// 全ての数を昇順で返す。
    pub fn all() -> [Self; Self::NUM] {
        std::array::from_fn(|i| unsafe { Self::new_unchecked((i + 1) as u8) })
    }
}
