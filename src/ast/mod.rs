#[cfg(test)]
#[macro_use]
mod test_utils {
    macro_rules! curry_parse_macros {
        // HACK: because of https://github.com/rust-lang/rust/issues/35853 we
        // need to pass `$` as an argument to macro.
        ($d:tt $AstNode:ident) => {
            macro_rules! parse {
                ($d ($d t:tt)+) => {
                    syn::parse_str::<$AstNode>(stringify!($d ($d t)*))
                };
            }

            macro_rules! parse_ok {
                ($d ($d t:tt)*) => {
                    parse!($d ($d t)*).unwrap()
                };
            }

            macro_rules! parse_err {
                ($d ($d t:tt)*) => {
                    format!("{}", parse!($d ($d t)*).unwrap_err())
                };
            }
        };
    }
}

mod patterns;
mod arm;
