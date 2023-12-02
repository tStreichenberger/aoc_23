//! Adding Extension methods

use std::{
    fmt::Debug,
    marker::PhantomData,
    str::{
        FromStr,
        Lines,
    },
    iter::Sum,
};

pub trait StringExt {
    fn parsed_lines<T>(&self) -> ParsedLinesIter<'_, T>;
}

impl StringExt for String {
    fn parsed_lines<T>(&self) -> ParsedLinesIter<'_, T> {
        ParsedLinesIter(self.lines(), PhantomData)
    }
}

pub struct ParsedLinesIter<'a, T>(Lines<'a>, PhantomData<T>);

impl<'a, T> Iterator for ParsedLinesIter<'a, T>
where
    T: FromStr,
    T::Err: Debug,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|line| line.parse().unwrap())
    }
}







pub trait IterExt: Iterator {
    fn sum_by<B, F>(self, f: F) -> B
    where
        Self: Sized,
        F: FnMut(Self::Item) -> B,
        B: Sum<B>
    {
        self.map(f).sum()
    }
}

impl<I: Iterator> IterExt for I {}