pub fn parse_nums<Output, Item>(input: impl AsRef<str>) -> Output
where
    Output: FromIterator<Item>,
    Item: std::str::FromStr,
    <Item as std::str::FromStr>::Err: std::fmt::Debug,
{
    input
        .as_ref()
        .split(|c: char| c.is_ascii_whitespace() || c.is_ascii_punctuation())
        .filter_map(|n| n.parse().ok())
        .collect()
}
