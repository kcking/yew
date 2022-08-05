mod cmark;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use proc_macro::{TokenStream, TokenTree};

use self::cmark::parse_commonmark;

lazy_static::lazy_static! {
static ref GLOBAL_STYLE : Arc<Mutex<HashMap<String, String>>> = {
    Default::default()
};
}

pub fn mdx_style(input: TokenStream) -> TokenStream {
    GLOBAL_STYLE
        .lock()
        .unwrap()
        .insert("h3".into(), "MyHeading3".into());
    quote::quote! {}.into()
}

pub fn mdx(input: TokenStream) -> TokenStream {
    let parsed = input
        .into_iter()
        .map(|token| match token {
            lit @ TokenTree::Literal(_) => {
                let mdx_str = lit.to_string();
                let mdx_str = mdx_str
                    .strip_prefix("r#\"")
                    .unwrap()
                    .strip_suffix("\"#")
                    .unwrap();
                // let parsed = parse_mdx().parse(mdx_str).unwrap();
                // // dbg!(&parsed);
                // let evaled = parsed.eval_outer();
                // // dbg!(&evaled.to_string());
                // evaled
                parse_commonmark(&mdx_str)
            }
            _ => panic!("mdx! expected literal"),
        })
        .collect::<TokenStream>();

    parsed
}
