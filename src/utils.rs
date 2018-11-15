pub use pin_utils::pin_mut;

#[macro_export]
macro_rules! pin_let {
    ($ident:ident = $expr:expr) => {
        let $ident = $expr;
        $crate::utils::pin_mut!($ident);
    };
}
