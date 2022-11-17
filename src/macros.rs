/// 引数が必ず真となることを表明し、最適化を促進する。
/// 表明が満たされない場合の挙動は未定義。
///
/// デバッグビルドでは `assert!($cond)` と等価。
/// リリースビルドでは `if !$cond { std::hint::unreachable_unchecked() }` と等価。
macro_rules! assert_unchecked {
    ($cond:expr) => {{
        #[cfg(debug_assertions)]
        {
            const unsafe fn __needs_unsafe() {}
            __needs_unsafe();
            ::std::assert!($cond);
        }
        #[cfg(not(debug_assertions))]
        {
            if !$cond {
                ::std::hint::unreachable_unchecked();
            }
        }
    }};
}
pub(crate) use assert_unchecked;
