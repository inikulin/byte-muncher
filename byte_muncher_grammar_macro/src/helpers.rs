macro_rules! parse_if_present {
    ($input:ident, { $t:tt }) => {
        if $input.peek(syn::Token! { $t }) {
            $input.parse::<syn::Token! { $t }>()?;
            true
        } else {
            false
        }
    };
}

macro_rules! parse2_if_present {
    ($input:ident, {$t1:tt}, {$t2:tt}) => {
        if $input.peek(syn::Token! { $t1 }) && $input.peek2(syn::Token! { $t2 }) {
            $input.parse::<syn::Token! { $t1 }>()?;
            $input.parse::<syn::Token! { $t2 }>()?;
            true
        } else {
            false
        }
    };
}

macro_rules! parse3_if_present {
    ($input:ident, {$t1:tt}, {$t2:tt}, {$t3:tt}) => {
        if $input.peek(syn::Token! { $t1 })
            && $input.peek2(syn::Token! { $t2 })
            && $input.peek3(syn::Token! { $t3 })
        {
            $input.parse::<syn::Token! { $t1 }>()?;
            $input.parse::<syn::Token! { $t2 }>()?;
            $input.parse::<syn::Token! { $t3 }>()?;
            true
        } else {
            false
        }
    };
}

#[cfg(test)]
#[macro_use]
mod test_utils {
    // NOTE: rustfmt doesn't play well with the hack below
    #[rustfmt::skip]
    macro_rules! curry_parse_macros {
        // HACK: because of https://github.com/rust-lang/rust/issues/35853 we
        // need to pass `$` as an argument to macro.
        ($d:tt $AstNode:ident) => {
            macro_rules! parse {
                ($d ($d t:tt)*) => {
                    syn::parse_str::<$AstNode>(stringify!($d ($d t)*))
                };
            }

            macro_rules! parse_ok {
                ($d ($d t:tt)*) => {
                    parse!($d ($d t)*).unwrap()
                };
            }

            #[allow(unused_macros)]
            macro_rules! parse_err {
                ($d ($d t:tt)*) => {
                    format!("{}", parse!($d ($d t)*).unwrap_err())
                };
            }
        };
    }

    macro_rules! code_str {
        ($($t:tt)*) => {
            // NOTE: parse-compile to discard formating
            stringify!($($t)*)
                .parse::<proc_macro2::TokenStream>()
                .expect("Rust code parsing failed")
                .to_string()
        };
    }

    macro_rules! act {
        ($name:expr) => {
            crate::ActionCall::UserDefined {
                name: $name.into(),
                args: vec![],
                with_error_check: false,
            }
        };
    }

    macro_rules! lit {
        ($t:tt) => {
            syn::parse_str::<syn::Lit>(stringify!($t)).unwrap()
        };
    }
}
