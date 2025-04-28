extern crate proc_macro;

/// Repeats a code block, substituting a value into each block.
///
/// ## With List of Values
///
/// The list of values is an literal array of unsuffixed integers.
///
/// ```rust
/// # extern crate proc_macro;
/// # use literal_loop::repeat_for;
/// repeat_for!(var in [1, 2, 3] => {
///     /* ... */
/// });
/// ```
///
/// ## With Range
///
/// The range is an literal inclusive range of integers.
///
/// ```rust
/// # extern crate proc_macro;
/// # use literal_loop::repeat_for;
/// # let (lower, upper) = (1, 2);
/// repeat_for!(var in (1..=20) => {
///     /* ... */
/// });
#[proc_macro]
pub fn repeat_for(stream: ::proc_macro::TokenStream) -> ::proc_macro::TokenStream {
    let input = ::syn::parse_macro_input!(stream as Loop);

    let loop_var_ident = &input.ident;

    let mut blocks = ::std::vec::Vec::<::proc_macro::TokenStream>::new();

    match input.values {
        LoopValues::Set(values) => {
            let iter_values = values.into_iter();

            for value in iter_values {
                let value = match value {
                    ::syn::Lit::Int(l) => match l.base10_parse::<i128>() {
                        Ok(i) => i,
                        Err(e) => return e.into_compile_error().into(),
                    },
                    _ => {
                        return syn::Error::new(value.span(), "List values must be integers")
                            .into_compile_error()
                            .into();
                    }
                };

                blocks.push(substituted_block(
                    input.block.clone(),
                    loop_var_ident,
                    ::proc_macro::Literal::i128_unsuffixed(value),
                ));
            }
        }
        LoopValues::Range(lower, _, upper) => {
            let lower_limit = match lower {
                ::syn::Lit::Int(l) => match l.base10_parse::<i128>() {
                    Ok(i) => i,
                    Err(e) => return e.into_compile_error().into(),
                },
                _ => {
                    return syn::Error::new(lower.span(), "Range bounds must be integers")
                        .into_compile_error()
                        .into();
                }
            };

            let upper_limit = match upper {
                ::syn::Lit::Int(l) => match l.base10_parse::<i128>() {
                    Ok(i) => i,
                    Err(e) => return e.into_compile_error().into(),
                },
                _ => {
                    return syn::Error::new(upper.span(), "Range bounds must be integers")
                        .into_compile_error()
                        .into();
                }
            };

            for value in lower_limit..=upper_limit {
                blocks.push(substituted_block(
                    input.block.clone(),
                    loop_var_ident,
                    ::proc_macro::Literal::i128_unsuffixed(value),
                ));
            }
        }
    }

    blocks
        .into_iter()
        .fold(::proc_macro::TokenStream::new(), |mut stream, block| {
            stream.extend(block);
            stream
        })
}

struct Loop {
    ident: ::syn::Ident,
    _in: ::syn::Token![in],
    values: LoopValues,
    block: ::syn::Block,
}

#[allow(dead_code)]
enum LoopValues {
    Set(::syn::punctuated::Punctuated<::syn::Lit, ::syn::Token![,]>),
    Range(::syn::Lit, ::syn::Token![..=], ::syn::Lit),
}

impl ::syn::parse::Parse for Loop {
    fn parse(input: ::syn::parse::ParseStream) -> ::syn::Result<Self> {
        let ident = input.parse()?;
        let _in = input.parse()?;

        let lookahead = input.lookahead1();

        let values = if lookahead.peek(::syn::token::Bracket) {
            let content;
            let _ = ::syn::bracketed!(content in input);
            LoopValues::Set(content.parse_terminated(::syn::Lit::parse, ::syn::Token![,])?)
        } else if lookahead.peek(::syn::token::Paren) {
            let content;
            let _ = ::syn::parenthesized!(content in input);

            LoopValues::Range(content.parse()?, content.parse()?, content.parse()?)
        } else {
            Err(lookahead.error())?
        };

        let _: ::syn::Token![=>] = input.parse()?;

        let block = input.parse()?;

        Ok(Self {
            ident,
            _in,
            values,
            block,
        })
    }
}

fn substituted_block(
    block: ::syn::Block,
    ident: &::syn::Ident,
    value: ::proc_macro::Literal,
) -> ::proc_macro::TokenStream {
    block
        .stmts
        .iter()
        .map(|statement| ::quote::ToTokens::into_token_stream(statement))
        .map(|stream| replace_tokens(stream.into(), ident, value.clone()))
        .fold(::proc_macro::TokenStream::new(), |mut stream, block| {
            stream.extend(block);
            stream
        })
}

fn replace_tokens(
    stream: ::proc_macro::TokenStream,
    ident: &::syn::Ident,
    value: ::proc_macro::Literal,
) -> ::proc_macro::TokenStream {
    stream
        .into_iter()
        .map(|tt| match tt {
            ::proc_macro::TokenTree::Ident(ref i) if i.to_string() == ident.to_string() => {
                ::proc_macro::TokenTree::Literal(value.clone())
            }
            ::proc_macro::TokenTree::Group(g) => {
                ::proc_macro::TokenTree::Group(::proc_macro::Group::new(
                    g.delimiter(),
                    replace_tokens(g.stream(), ident, value.clone()),
                ))
            }
            other => other,
        })
        .collect()
}
