use derive_more::{Debug, Display};
use pathfinding::prelude::Matrix;

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

pub struct MatrixIter<'a, T, I, O> {
    matrix: &'a Matrix<T>,
    index: usize,
    iterate: I,
    _phantom: std::marker::PhantomData<O>,
}

impl<'a, T, I, O> MatrixIter<'a, T, I, O> {
    fn new(matrix: &'a Matrix<T>, iterate: I) -> Self {
        Self {
            matrix,
            index: 0,
            iterate,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<'a, T, I, O> Iterator for MatrixIter<'a, T, I, O>
where
    I: Fn(&'a Matrix<T>, usize) -> Option<O>,
{
    type Item = O;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = (self.iterate)(self.matrix, self.index) {
            self.index += 1;
            return Some(item);
        }

        None
    }
}

pub trait MatrixExt {
    type Item;
    type Iter;

    fn row(&self, row: usize) -> Vec<&Self::Item>;

    fn column(&self, column: usize) -> Vec<&Self::Item>;

    fn columns_iter(&self) -> MatrixIter<Self::Item, Self::Iter, Vec<&Self::Item>>;

    fn rows_iter(&self) -> MatrixIter<Self::Item, Self::Iter, Vec<&Self::Item>>;
}

impl<T> MatrixExt for Matrix<T> {
    type Item = T;
    type Iter = fn(&Matrix<T>, usize) -> Option<Vec<&T>>;

    fn row(&self, row: usize) -> Vec<&Self::Item> {
        (0..self.columns)
            .map(|column| &self[(row, column)])
            .collect()
    }

    fn column(&self, column: usize) -> Vec<&Self::Item> {
        (0..self.rows).map(|row| &self[(row, column)]).collect()
    }

    fn columns_iter(&self) -> MatrixIter<Self::Item, Self::Iter, Vec<&Self::Item>> {
        MatrixIter::new(self, |matrix, index| {
            if index < matrix.columns {
                Some((0..matrix.rows).map(|row| &matrix[(row, index)]).collect())
            } else {
                None
            }
        })
    }

    fn rows_iter(&self) -> MatrixIter<Self::Item, Self::Iter, Vec<&Self::Item>> {
        MatrixIter::new(self, |matrix, index| {
            if index < matrix.rows {
                Some(
                    (0..matrix.columns)
                        .map(|column| &matrix[(index, column)])
                        .collect(),
                )
            } else {
                None
            }
        })
    }
}
