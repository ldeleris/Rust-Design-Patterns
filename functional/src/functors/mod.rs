//! `functors` module.
//!
//! # Example
//!
//! ```rust,ignore
//! use functional::functors::*;
//! ```


pub trait Functor {
    type T;
    fn map<Y, I: Iterator<Item = Self::T>>(l: &I, f: fn(Self::T) -> Y) -> Iterator<Item = Y>;
}
