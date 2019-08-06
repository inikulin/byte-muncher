use super::*;
use crate::dsl::Compile;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{Ident, IntSuffix, LitInt};

impl Compile for AliasPattern {
    fn compile(&self) -> TokenStream2 {
        match self {
            AliasPattern::Alpha => quote! { Some(b'a'..=b'z') | Some(b'A'..=b'Z') },
            AliasPattern::AlphaLower => quote! { Some(b'a'..=b'z') },
            AliasPattern::AlphaUpper => quote! { Some(b'A'..=b'Z') },
            AliasPattern::Digit => quote! { Some(b'0'..=b'9') },
            AliasPattern::Whitespace => quote! {
                Some(b' ') | Some(b'\n') | Some(b'\r') | Some(b'\t') | Some(b'\x0C')
            },
        }
    }
}

impl Compile for Pattern {
    fn compile(&self) -> TokenStream2 {
        match self {
            Pattern::Byte(b) => {
                let lit = LitInt::new((*b).into(), IntSuffix::U8, Span::call_site());

                quote! { Some(#lit) }
            }
            Pattern::Alias(s) => s.compile(),
            Pattern::Condition(name) => {
                let condition = Ident::new(name, Span::call_site());

                quote! { Some(b) if #condition(b) }
            }
            Pattern::Any => quote! { _ },
            _ => unreachable!(concat![
                "Compilation of the rest of patterns requires modification of match arm RHS",
                "and, therefore, they should be compiled elsewhere"
            ]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    curry_compile_macro!($Pattern);

    #[test]
    fn compile_byte_pattern() {
        assert_eq!(compile! { 'a' }, code_str! { Some(97u8) });
    }

    #[test]
    fn compile_alias_pattern() {
        assert_eq!(
            compile! { alpha },
            code_str! { Some(b'a'..=b'z') | Some(b'A'..=b'Z') }
        );

        assert_eq!(compile! { alpha_lo }, code_str! { Some( b'a'..=b'z') });
        assert_eq!(compile! { alpha_up }, code_str! { Some(b'A'..=b'Z') });
        assert_eq!(compile! { digit }, code_str! { Some(b'0'..=b'9') });

        assert_eq!(
            compile! { ws },
            code_str! { Some(b' ') | Some(b'\n') | Some(b'\r') | Some(b'\t') | Some(b'\x0C') }
        );
    }

    #[test]
    fn compile_condition() {
        assert_eq!(compile! { if foo }, code_str! { Some(b) if foo(b) });
    }

    #[test]
    fn compile_any() {
        assert_eq!(compile! { _ }, code_str! { _ });
    }
}
