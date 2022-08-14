/// Builds `Vec` of `Vec`
///
/// ```
/// let vec = vv![[1], [2, 3], [4]];
/// assert_eq!(vec, vec![vec![1], vec![2, 3], vec![4]]);
/// ```
#[macro_export]
macro_rules! vv {
    ($($x:tt),*) => { vec![$(vec!$x),*] };
}

/// Builds `Vec` of `String`
///
/// ```
/// let vec = vs!["a", "b", "c"];
/// assert_eq!(vec, vec!["a".to_string(), "b".to_string(), "c".to_string()]);
/// ```
#[macro_export]
macro_rules! vs {
    ($($x:expr),*) => { vec![$($x.to_string()),*] };
}
