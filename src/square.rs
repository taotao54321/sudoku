use crate::macros::assert_unchecked;

/// マス。
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Square(u8);

impl Square {
    pub const MIN_VALUE: u8 = 0;
    pub const MAX_VALUE: u8 = 80;

    /// マスの総数。
    pub const NUM: usize = 81;

    /// 内部値を指定してマスを作る。
    pub fn new(inner: u8) -> Option<Self> {
        matches!(inner, Self::MIN_VALUE..=Self::MAX_VALUE)
            .then(|| unsafe { Self::new_unchecked(inner) })
    }

    /// 内部値を指定してマスを作る。
    ///
    /// # Safety
    ///
    /// `inner` は有効値でなければならない。
    pub unsafe fn new_unchecked(inner: u8) -> Self {
        assert_unchecked!(matches!(inner, Self::MIN_VALUE..=Self::MAX_VALUE));
        Self(inner)
    }

    /// 列と行を指定してマスを作る。
    pub fn from_col_row(col: Col, row: Row) -> Self {
        let inner = 9 * row.get() + col.get();
        unsafe { Self::new_unchecked(inner) }
    }

    /// 内部値を返す。
    pub fn get(self) -> u8 {
        self.0
    }

    /// マスが属する列を返す。
    pub fn col(self) -> Col {
        #[rustfmt::skip]
        const TABLE: [u8; Square::NUM] = [
            0, 1, 2, 3, 4, 5, 6, 7, 8,
            0, 1, 2, 3, 4, 5, 6, 7, 8,
            0, 1, 2, 3, 4, 5, 6, 7, 8,
            0, 1, 2, 3, 4, 5, 6, 7, 8,
            0, 1, 2, 3, 4, 5, 6, 7, 8,
            0, 1, 2, 3, 4, 5, 6, 7, 8,
            0, 1, 2, 3, 4, 5, 6, 7, 8,
            0, 1, 2, 3, 4, 5, 6, 7, 8,
            0, 1, 2, 3, 4, 5, 6, 7, 8,
        ];

        let inner = TABLE[usize::from(self.0)];
        unsafe { Col::new_unchecked(inner) }
    }

    /// マスが属する行を返す。
    pub fn row(self) -> Row {
        #[rustfmt::skip]
        const TABLE: [u8; Square::NUM] = [
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            1, 1, 1, 1, 1, 1, 1, 1, 1,
            2, 2, 2, 2, 2, 2, 2, 2, 2,
            3, 3, 3, 3, 3, 3, 3, 3, 3,
            4, 4, 4, 4, 4, 4, 4, 4, 4,
            5, 5, 5, 5, 5, 5, 5, 5, 5,
            6, 6, 6, 6, 6, 6, 6, 6, 6,
            7, 7, 7, 7, 7, 7, 7, 7, 7,
            8, 8, 8, 8, 8, 8, 8, 8, 8,
        ];

        let inner = TABLE[usize::from(self.0)];
        unsafe { Row::new_unchecked(inner) }
    }

    /// マスが属するブロックを返す。
    pub fn block(self) -> Block {
        #[rustfmt::skip]
        const TABLE: [u8; Square::NUM] = [
            0, 0, 0, 1, 1, 1, 2, 2, 2,
            0, 0, 0, 1, 1, 1, 2, 2, 2,
            0, 0, 0, 1, 1, 1, 2, 2, 2,
            3, 3, 3, 4, 4, 4, 5, 5, 5,
            3, 3, 3, 4, 4, 4, 5, 5, 5,
            3, 3, 3, 4, 4, 4, 5, 5, 5,
            6, 6, 6, 7, 7, 7, 8, 8, 8,
            6, 6, 6, 7, 7, 7, 8, 8, 8,
            6, 6, 6, 7, 7, 7, 8, 8, 8,
        ];

        let inner = TABLE[usize::from(self.0)];
        unsafe { Block::new_unchecked(inner) }
    }

    /// 全てのマスを昇順で返す。
    pub fn all() -> [Self; Self::NUM] {
        std::array::from_fn(|i| unsafe { Self::new_unchecked(i as u8) })
    }

    /// 指定した列に属する全てのマスを昇順で返す。
    pub fn col_all(col: Col) -> [Self; 9] {
        let base = col.get();

        std::array::from_fn(|i| {
            let inner = base + 9 * i as u8;
            unsafe { Self::new_unchecked(inner) }
        })
    }

    /// 指定した行に属する全てのマスを昇順で返す。
    pub fn row_all(row: Row) -> [Self; 9] {
        let base = 9 * row.get();

        std::array::from_fn(|i| {
            let inner = base + i as u8;
            unsafe { Self::new_unchecked(inner) }
        })
    }

    /// 指定したブロックに属する全てのマスを昇順で返す。
    pub fn block_all(block: Block) -> [Self; 9] {
        const BASE_TABLE: [u8; Block::NUM] = [0, 3, 6, 27, 30, 33, 54, 57, 60];
        const OFFSET_TABLE: [u8; 9] = [0, 1, 2, 9, 10, 11, 18, 19, 20];

        let base = BASE_TABLE[usize::from(block.get())];

        std::array::from_fn(|i| {
            let inner = base + OFFSET_TABLE[i];
            unsafe { Self::new_unchecked(inner) }
        })
    }
}

/// 列。
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Col(u8);

impl Col {
    pub const MIN_VALUE: u8 = 0;
    pub const MAX_VALUE: u8 = 8;

    /// 列の総数。
    pub const NUM: usize = 9;

    /// 内部値を指定して列を作る。
    pub fn new(inner: u8) -> Option<Self> {
        matches!(inner, Self::MIN_VALUE..=Self::MAX_VALUE)
            .then(|| unsafe { Self::new_unchecked(inner) })
    }

    /// 内部値を指定して列を作る。
    ///
    /// # Safety
    ///
    /// `inner` は有効値でなければならない。
    pub unsafe fn new_unchecked(inner: u8) -> Self {
        assert_unchecked!(matches!(inner, Self::MIN_VALUE..=Self::MAX_VALUE));
        Self(inner)
    }

    /// 内部値を返す。
    pub fn get(self) -> u8 {
        self.0
    }

    /// 全ての列を昇順で返す。
    pub fn all() -> [Self; Self::NUM] {
        std::array::from_fn(|i| unsafe { Self::new_unchecked(i as u8) })
    }
}

/// 行。
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Row(u8);

impl Row {
    pub const MIN_VALUE: u8 = 0;
    pub const MAX_VALUE: u8 = 8;

    /// 行の総数。
    pub const NUM: usize = 9;

    /// 内部値を指定して行を作る。
    pub fn new(inner: u8) -> Option<Self> {
        matches!(inner, Self::MIN_VALUE..=Self::MAX_VALUE)
            .then(|| unsafe { Self::new_unchecked(inner) })
    }

    /// 内部値を指定して行を作る。
    ///
    /// # Safety
    ///
    /// `inner` は有効値でなければならない。
    pub unsafe fn new_unchecked(inner: u8) -> Self {
        assert_unchecked!(matches!(inner, Self::MIN_VALUE..=Self::MAX_VALUE));
        Self(inner)
    }

    /// 内部値を返す。
    pub fn get(self) -> u8 {
        self.0
    }

    /// 全ての行を昇順で返す。
    pub fn all() -> [Self; Self::NUM] {
        std::array::from_fn(|i| unsafe { Self::new_unchecked(i as u8) })
    }
}

/// ブロック。
///
/// 配置は以下の通り:
///
/// ```text
/// 0 1 2
/// 3 4 5
/// 6 7 8
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Block(u8);

impl Block {
    pub const MIN_VALUE: u8 = 0;
    pub const MAX_VALUE: u8 = 8;

    /// ブロックの総数。
    pub const NUM: usize = 9;

    /// 内部値を指定してブロックを作る。
    pub fn new(inner: u8) -> Option<Self> {
        matches!(inner, Self::MIN_VALUE..=Self::MAX_VALUE)
            .then(|| unsafe { Self::new_unchecked(inner) })
    }

    /// 内部値を指定してブロックを作る。
    ///
    /// # Safety
    ///
    /// `inner` は有効値でなければならない。
    pub unsafe fn new_unchecked(inner: u8) -> Self {
        assert_unchecked!(matches!(inner, Self::MIN_VALUE..=Self::MAX_VALUE));
        Self(inner)
    }

    /// 内部値を返す。
    pub fn get(self) -> u8 {
        self.0
    }

    /// 全てのブロックを昇順で返す。
    pub fn all() -> [Self; Self::NUM] {
        std::array::from_fn(|i| unsafe { Self::new_unchecked(i as u8) })
    }
}
