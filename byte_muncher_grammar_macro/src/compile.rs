use proc_macro2::TokenStream as TokenStream2;

pub trait Compile {
    fn compile(&self) -> TokenStream2;
}
