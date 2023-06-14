
#![doc = include_str!("../README.md")]

#![no_std]

#[cfg(doc)]
extern crate std;

use core::ops::Deref;
use core::cmp::*;
use core::marker::PhantomData;

/// A wrapper around a type, that implements the [Ord] trait in terms of the `F` function
pub struct OrdBy<T, F>{
    content: T,
    func: F
}

impl<T, F> Deref for OrdBy<T, F> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.content
    }
}

impl<T, F: Fn(&T, &T) -> Ordering> OrdBy<T, F> {
    /// Creates a new `OrdBy`, wrapping `T`, using the `F` compare function
    pub fn new(content: T, func: F) -> Self {
        Self {
            content,
            func
        }
    }
    /// Unwraps the `OrdBy`, returning the inner `T`
    pub fn into_inner(self) -> T {
        self.content
    }
    /// Borrows `T` from the `OrdBy`
    pub fn borrow(&self) -> &T {
        &self.content
    }
}

impl<T, F: Fn(&T, &T) -> Ordering> PartialOrd for OrdBy<T, F> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T, F> PartialEq for OrdBy<T, F> {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl<T, F> Eq for OrdBy<T, F> {}

impl<T, F: Fn(&T, &T) -> Ordering> Ord for OrdBy<T, F> {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.func)(&self.content, &other.content)
    }
}

/// A trait for any [Iterator] exposing the [OrdByIterTrait::ord_by] method
pub trait OrdByIterTrait<T, F> {
    type Source: Iterator<Item=T>;

    /// Wraps every item in an [OrdBy]
    fn ord_by(self, cmp_func: F) -> OrdByIter<Self::Source, T, F>;
}

impl<I, T, F> OrdByIterTrait<T, F> for I
    where I: Iterator<Item=T>,
    F: Fn(&T, &T) -> Ordering
{
    type Source = I;

    fn ord_by(self, cmp_func: F) -> OrdByIter<I, T, F> {
        OrdByIter{source: self, cmp_func, phantom: PhantomData}
    }
}

impl<I, T, F> Iterator for OrdByIter<I, T, F>
    where
    I: Iterator<Item=T>,
    F: Fn(&T, &T) -> Ordering + Clone
{
    type Item = OrdBy<T, F>;

    fn next(&mut self) -> Option<Self::Item> {
        self.source.next().map(|item| OrdBy::new(item, self.cmp_func.clone()))
    }
}

/// An [Iterator] returned by [OrdByIterTrait::ord_by]
pub struct OrdByIter<I, T, F> {
    source: I,
    cmp_func: F,
    phantom: PhantomData<T>,
}
