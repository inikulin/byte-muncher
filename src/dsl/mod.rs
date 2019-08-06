#[macro_use]
mod helpers;

mod directives;
mod grammar;
mod match_arm;
mod patterns;
mod state;

pub use self::directives::*;
pub use self::grammar::*;
pub use self::match_arm::*;
pub use self::patterns::*;
pub use self::state::*;
use proc_macro2::TokenStream as TokenStream2;

pub trait Compile {
    fn compile(&self) -> TokenStream2;
}