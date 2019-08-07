#[macro_use]
mod helpers;

mod arm;
mod directives;
mod grammar;
mod state;

pub use self::arm::*;
pub use self::directives::*;
pub use self::grammar::*;
pub use self::state::*;
use proc_macro2::TokenStream as TokenStream2;

pub trait Compile {
    fn compile(&self) -> TokenStream2;
}
