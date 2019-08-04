#[macro_use]
mod helpers;

mod directives;
mod match_arm;
mod patterns;
mod state;

use proc_macro2::Span;
use std::cmp::PartialEq;
use std::ops::Deref;
use syn::Error as ParseError;

pub use self::directives::*;
pub use self::match_arm::*;
pub use self::patterns::*;
pub use self::state::*;

pub struct WithSpan<T> {
    pub node: T,
    pub span: Span,
}

impl<T> WithSpan<T> {
    pub fn new(node: T, span: Span) -> Self {
        WithSpan { node, span }
    }

    pub fn error(&self, msg: &str) -> ParseError {
        ParseError::new(self.span, msg)
    }
}

impl<T: PartialEq> PartialEq<WithSpan<T>> for WithSpan<T> {
    fn eq(&self, other: &WithSpan<T>) -> bool {
        self.node == other.node
    }
}

impl<T> Deref for WithSpan<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.node
    }
}
