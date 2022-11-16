use anyhow::{bail, ensure};

use crate::number::Number;
use crate::square::*;

/// 盤面。`Square` でインデックスアクセスできる。
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Board([Option<Number>; 81]);

impl Board {
    /// 空の盤面を返す。
    pub fn empty() -> Self {
        Self([None; 81])
    }

    /// 内部配列を与えて盤面を作る。盤面が不正ならエラーを返す。
    pub fn new(inner: [Option<Number>; 81]) -> anyhow::Result<Self> {
        let is_ok = |sqs: [Square; 9]| -> bool {
            let mut mask = 0_u32;
            for sq in sqs {
                let Some(num) = inner[usize::from(sq.get())] else {
                    continue;
                };
                let bit = 1 << num.get();
                if (mask & bit) != 0 {
                    return false;
                }
                mask |= bit;
            }
            true
        };

        for col in Col::all() {
            ensure!(is_ok(Square::col_all(col)), "col {} is illegal", col.get());
        }
        for row in Row::all() {
            ensure!(is_ok(Square::row_all(row)), "row {} is illegal", row.get());
        }
        for block in Block::all() {
            ensure!(
                is_ok(Square::block_all(block)),
                "block {} is illegal",
                block.get()
            );
        }

        Ok(Self(inner))
    }

    /// 盤面が既に解けているかどうかを返す。
    pub fn is_solved(&self) -> bool {
        Square::all().into_iter().all(|sq| self[sq].is_some())
    }
}

impl std::ops::Index<Square> for Board {
    type Output = Option<Number>;

    fn index(&self, sq: Square) -> &Self::Output {
        &self.0[usize::from(sq.get())]
    }
}

impl std::ops::IndexMut<Square> for Board {
    fn index_mut(&mut self, sq: Square) -> &mut Self::Output {
        &mut self.0[usize::from(sq.get())]
    }
}

impl std::str::FromStr for Board {
    type Err = anyhow::Error;

    /// 盤面をパースする。
    ///
    /// ちょうど 81 個の数字を読み取る。空白は無視される。
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut inner = [None::<Number>; 81];

        let mut i = 0;
        for c in s.chars().filter(|&c| !c.is_ascii_whitespace()) {
            ensure!(i < 81, "too many numbers");

            let c @ '0'..='9' = c else {
                bail!("invalid char: {c}");
            };
            let digit = c.to_digit(10).unwrap() as u8;
            inner[i] = Number::new(digit);

            i += 1;
        }
        ensure!(i == 81, "too few numbers");

        Self::new(inner)
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write as _;

        for row in Row::all() {
            for col in Col::all() {
                if col.get() != 0 {
                    f.write_char(' ')?;
                }
                let sq = Square::from_col_row(col, row);
                let n = self[sq].map_or(0, Number::get);
                write!(f, "{}", n)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
