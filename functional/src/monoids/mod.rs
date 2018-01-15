//! `monoids` module.
//!
//! # Example
//!
//! ```rust,ignore
//! use functional::monoids::*;
//! ```

use std;

pub trait Monoid {
    type T;
    fn op(&self, l: Self::T, r: Self::T) -> Self::T;
    fn zero(&self) -> Self::T;
}

pub struct IntAddition;
impl Monoid for IntAddition {
    type T = i32;
    fn op(&self, l: i32, r: i32) -> i32 {
        l + r
    }
    fn zero(&self) -> i32 {
        0
    }
}


pub struct IntMultiplication;
impl Monoid for IntMultiplication {
    type T = i32;
    fn op(&self, l: i32, r: i32) -> i32 {
        l * r
    }
    fn zero(&self) -> i32 {
        1
    }
}

pub struct StringConcatenation;
impl Monoid for StringConcatenation {
    type T = String;
    fn op(&self, l: String, r: String) -> String {
        let mut tmp = l.clone();
        tmp.push_str(&r[..]);
        tmp.clone()
    }
    fn zero(&self) -> String {
        String::from("")
    }
}


pub trait MonoidOperations {
    type T;
    fn fold(vec: Vec<Self::T>) -> Self::T;
}

impl MonoidOperations for IntAddition {
    type T = i32;
    fn fold(vec: Vec<i32>) -> i32 {
        vec.into_iter().fold(IntAddition.zero(), |x, y| IntAddition.op(x, y))
    }
}

pub struct MOpt;
impl MOpt {
    pub fn fold<Y: Clone, M: Monoid<T = Y> + 'static>(vec: Vec<Y>, m: &M) -> Y {
        MOpt::fold_map(vec, m, MOpt::identity)
    }

    pub fn fold_map<A, B, M>(vec: Vec<A>, m: &M, f: fn(A) -> B) -> B
        where A: Clone,
              B: Clone,
              M: Monoid<T = B> + 'static
    {
        vec.into_iter().fold((*m).zero(), move |x, y| (*m).op(x, f(y)))
    }

    pub fn identity<A: Clone>(x: A) -> A {
        x
    }

    pub fn balance_fold<A, B, M>(vec: Vec<A>, m: &M, f: fn(A) -> B) -> B
        where A: Clone,
              B: Clone,
              M: Monoid<T = B> + 'static
    {
        if vec.is_empty() {
            (*m).zero()
        } else if vec.len() == 1 {
            f(vec[0].clone())
        } else {
            let (left, right) = vec.split_at(vec.len() / 2);
            (*m).op(MOpt::balance_fold(Vec::from(left.clone()), m, f),
                    MOpt::balance_fold(Vec::from(right.clone()), m, f))
        }
    }
}
