use derive_more::{Debug, Display};

pub fn parse_nums<Output, Item>(input: impl AsRef<str>) -> Output
where
    Output: FromIterator<Item>,
    Item: std::str::FromStr,
    <Item as std::str::FromStr>::Err: std::fmt::Debug,
{
    input
        .as_ref()
        .split(|c: char| c != '-' && (c.is_ascii_whitespace() || c.is_ascii_punctuation()))
        .filter_map(|n| n.parse().ok())
        .collect()
}

#[derive(Debug, Copy, Clone, Display)]
#[display("({x},{y})")]
#[debug("({x},{y})")]
pub struct Point2D<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point2D<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> Point2D<T>
where
    T: Copy + Ord + std::ops::Sub<Output = T> + std::ops::Add<Output = T>,
{
    pub fn manhattan(&self, other: &Self) -> T {
        (self.x.max(other.x) - self.x.min(other.x)) + (self.y.max(other.y) - self.y.min(other.y))
    }
}

impl<T> From<(T, T)> for Point2D<T> {
    fn from((x, y): (T, T)) -> Self {
        Self::new(x, y)
    }
}
