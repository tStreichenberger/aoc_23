//! Adding Extension methods

use std::{
    fmt::Debug,
    iter::Sum,
    marker::PhantomData,
    str::{
        FromStr,
        Lines,
    },
};

pub trait StringExt {
    fn parsed_lines<T>(&self) -> ParsedIter<Lines<'_>, T>;
}

impl StringExt for str {
    fn parsed_lines<T>(&self) -> ParsedIter<Lines<'_>, T> {
        self.lines().parse_each()
    }
}

pub trait IterExt: Iterator {
    fn sum_by<B, F>(self, f: F) -> B
    where
        Self: Sized,
        F: FnMut(Self::Item) -> B,
        B: Sum<B>,
    {
        self.map(f).sum()
    }

    fn parse_each<T>(self) -> ParsedIter<Self, T>
    where
        Self: Sized,
    {
        ParsedIter(self, PhantomData)
    }
}

impl<I: Iterator> IterExt for I {}

pub struct ParsedIter<I, T>(I, PhantomData<T>);

impl<I, T> Iterator for ParsedIter<I, T>
where
    I: Iterator,
    I::Item: AsRef<str>,
    T: FromStr,
    T::Err: Debug,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|line| line.as_ref().parse().unwrap())
    }
}
