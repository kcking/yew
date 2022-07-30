mod cmark;

use proc_macro::{TokenStream, TokenTree};

use self::cmark::parse_commonmark;

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
