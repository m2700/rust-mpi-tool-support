use proc_macro2::{Delimiter, Ident, Punct, TokenStream, TokenTree};

enum ParseState {
    Begin,
    BufferPtr {
        buffer_ptr: Ident,
    },
    BufferPtrId {
        buffer_ptr: Ident,
        buffer_id: Ident,
    },
    BufferPtrArgId {
        buffer_ptr: Ident,
        buffer_id: Ident,
        buffer_ptr_id: Ident,
    },
}
use ParseState::*;

enum ArgumentDecl {
    BufferPtr {
        buffer_ptr: Ident,
        buffer_id: Ident,
        buffer_ptr_id: Ident,
        colon: Punct,
        buf_ptr_type: syn::Type,
        opt_comma: Option<Punct>,
    },
    
}
impl ArgumentDecl {
    fn parse(token_stream: Vec<TokenTree>, opt_comma: Option<Punct>) -> Self {
        let mut state = Begin;
        let mut token_stream_iter = token_stream.into_iter();
        for token in&mut token_stream_iter {
            state = match state {
                Begin => match token {
                    TokenTree::Ident(buffer_ptr) if buffer_ptr.to_string() == "buffer_ptr" => {
                        BufferPtr { buffer_ptr }
                    }
                    tkn => unexpected_token!(tkn),
                },
                BufferPtr { buffer_ptr } => match token {
                    TokenTree::Group(buffer_ptr_group)
                        if buffer_ptr_group.delimiter() == Delimiter::Parenthesis =>
                    {
                        match &*buffer_ptr_group.stream().into_iter().collect::<Vec<_>>() {
                            [TokenTree::Ident(buffer_id)] => BufferPtrId {
                                buffer_ptr,
                                buffer_id: buffer_id.clone(),
                            },
                            tkns => unexpected_tokens!(tkns),
                        }
                    }
                    tkn => unexpected_token!(tkn),
                },
                BufferPtrId {
                    buffer_ptr,
                    buffer_id,
                } => match token {
                    TokenTree::Ident(buffer_ptr_id) => BufferPtrArgId {
                        buffer_ptr,
                        buffer_id,
                        buffer_ptr_id,
                    },
                    tkn => unexpected_token!(tkn),
                },
                BufferPtrArgId {
                    buffer_ptr,
                    buffer_id,
                    buffer_ptr_id,
                } => match token {
                    TokenTree::Punct(colon) if colon.as_char() == ':' => {
                        let buf_ptr_type = syn::parse2(token_stream_iter.collect()).unwrap();
                        return Self::BufferPtr {
                            buffer_ptr,
                            buffer_id,
                            buffer_ptr_id,
                            colon,
                            buf_ptr_type,
                            opt_comma,
                        };
                    }
                    tkn => unexpected_token!(tkn),
                },
            };
        }
        unexpected_macro_end!()
    }
}

pub struct ArgLine {
    decls: Vec<ArgumentDecl>,
}
impl ArgLine {
    fn split_arguments(tokens: Vec<TokenTree>) -> (Vec<Vec<TokenTree>>, Vec<Punct>) {
        let mut eck_cnt = 0;
        let mut commas = vec![];
        let args = tokens
            .split(|token| match token {
                TokenTree::Punct(pct) if pct.as_char() == '<' => {
                    eck_cnt += 1;
                    false
                }
                TokenTree::Punct(pct) if eck_cnt > 0 && pct.as_char() == '>' => {
                    eck_cnt -= 1;
                    false
                }
                TokenTree::Punct(comma) if eck_cnt == 0 && comma.as_char() == ',' => {
                    commas.push(comma.clone());
                    true
                }
                _ => false,
            })
            .map(|slc| slc.to_vec())
            .collect();
        (args, commas)
    }
    pub fn parse(token_stream: TokenStream) -> Self {
        let (mut args, commas) = Self::split_arguments(token_stream.into_iter().collect());
        if let Some(ts) = args.last() {
            if ts.is_empty() {
                args.pop();
            }
        }
        Self {
            decls: args
                .into_iter()
                .enumerate()
                .map(|(idx, decl_ts)| ArgumentDecl::parse(decl_ts, commas.get(idx).cloned()))
                .collect(),
        }
    }
}
