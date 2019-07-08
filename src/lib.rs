#[cfg(test)]
#[macro_use]
mod test_utils {
    macro_rules! curry_parse_macros {
        ($AstNode:ident) => {
            macro_rules! parse {
                ($t:tt) => {
                    syn::parse_str::<$AstNode>(stringify!($t))
                };
            }

            macro_rules! parse_ok {
                ($t:tt) => {
                    parse!($t).unwrap()
                };
            }

            macro_rules! parse_err {
                ($t:tt) => {
                    format!("{}", parse!($t).unwrap_err())
                };
            }
        };
    }
}

mod patterns;
