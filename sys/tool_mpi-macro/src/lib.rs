use proc_macro2::{Delimiter, Group, Ident, Punct, TokenStream, TokenTree};

macro_rules! unexpected_token {
    ($tkn:expr) => {
        panic!("unexpected token: {}", $tkn)
    };
}
macro_rules! unexpected_tokens {
    ($tkns:expr) => {
        panic!(
            "unexpected tokens: {}",
            ($tkns).iter().cloned().collect::<TokenStream>()
        )
    };
}
macro_rules! unexpected_macro_end {
    () => {
        panic!("unexpected end of macro")
    };
}

mod arg_line;

use arg_line::*;

enum FnItemParseState {
    Begin,
    Fn_(Ident),
    FnId(Ident, Ident),
    FnIdArgline(Ident, Ident, ArgLine),
    FnIdArglineRpnt(Ident, Ident, ArgLine, Punct),
}
use FnItemParseState::*;

struct FnItem {
    fn_: Ident,
    fn_id: Ident,
    arg_line: ArgLine,
    dash: Punct,
    pnt: Punct,
    ret_type: syn::Type,
    semi_colon: Punct,
}
impl FnItem {
    fn parse(input_tokens: TokenStream) -> Self {
        let mut state = Begin;
        let mut token_iter = input_tokens.into_iter();
        for token in &mut token_iter {
            state = match state {
                Begin => match token {
                    TokenTree::Ident(fn_) => Fn_(fn_),
                    tkn => unexpected_token!(tkn),
                },
                Fn_(fn_) => match token {
                    TokenTree::Ident(fn_id) => FnId(fn_, fn_id),
                    tkn => unexpected_token!(tkn),
                },
                FnId(fn_, fn_id) => match token {
                    TokenTree::Group(group) if group.delimiter() == Delimiter::Parenthesis => {
                        FnIdArgline(fn_, fn_id, ArgLine::parse(group.stream()))
                    }
                    tkn => unexpected_token!(tkn),
                },
                FnIdArgline(fn_, fn_id, arg_line) => match token {
                    TokenTree::Punct(dash) if dash.as_char() == '-' => {
                        FnIdArglineRpnt(fn_, fn_id, arg_line, dash)
                    }
                    tkn => unexpected_token!(tkn),
                },
                FnIdArglineRpnt(fn_, fn_id, arg_line, dash) => match token {
                    TokenTree::Punct(pnt) if pnt.as_char() == '>' => {
                        let mut tokens: Vec<_> = token_iter.collect();
                        return match tokens.pop() {
                            Some(TokenTree::Punct(semi_colon)) if pnt.as_char() == ';' => Self {
                                fn_,
                                fn_id,
                                arg_line,
                                dash,
                                pnt,
                                ret_type: syn::parse2(tokens.into_iter().collect()).unwrap(),
                                semi_colon,
                            },
                            Some(tkn) => unexpected_token!(tkn),
                            None => unexpected_macro_end!(),
                        };
                    }
                    tkn => unexpected_token!(tkn),
                },
            };
        }
        unexpected_macro_end!()
    }
}

#[proc_macro]
pub fn impl_tool_helper_fn(input_tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input_tokens=TokenStream::from(input_tokens);
    let fn_item=FnItem::parse(input_tokens);
    unexpected_macro_end!()
}
