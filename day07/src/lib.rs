pub fn array_to_index_map<T, const N: usize>(items: [T; N]) -> impl Iterator<Item = (T, usize)> {
    items.into_iter().rev().enumerate().map(|(i, x)| (x, i))
}
