pub fn parse_nums<T>(input: impl AsRef<str>) -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    input
        .as_ref()
        .split(|c: char| c.is_ascii_whitespace() || c.is_ascii_punctuation())
        .filter_map(|n| n.parse().ok())
        .collect()
}
