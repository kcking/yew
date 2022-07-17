use chumsky::prelude::*;
use proc_macro::{TokenStream, TokenTree};
use quote::quote;

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
                let parsed = parse_mdx().parse(mdx_str).unwrap();
                dbg!(&parsed);
                parsed.eval()
            }
            _ => panic!("mdx! expected literal"),
        })
        .collect::<TokenStream>();

    // let mut toks = TokenStream::default();
    // toks.extend("r#\"".parse::<TokenStream>().unwrap());
    // toks.extend(parsed.collect::<TokenStream>());
    // toks.extend("\"#".parse::<TokenStream>().unwrap());
    // dbg!(toks.into())
    let s = format!("r#\"{}\"#", parsed);
    quote! {
        #s
    }
    .into()
}

fn parse_mdx() -> impl Parser<char, Expr, Error = Simple<char>> {
    // let start = just('r')
    //     .ignore_then(just('#'))
    //     .ignore_then(just('"'))
    //     .padded()
    //     .ignored();
    let expr = recursive(|expr| {
        let title = just('#')
            .padded()
            .ignore_then(expr.clone())
            .map(|t| Expr::Title(Box::new(t)));

        //  old text that goes to end of line, doesn't play nice when embedded in other things like a link
        // let text = take_until(text::newline()).map(|(s, _)| Expr::Text(s.into_iter().collect()));
        // let text = take_until(text::newline()).map(|(s, _)| Expr::Text(s.into_iter().collect()));
        let operators = &['(', ')', '[', ']'];
        let newlines = &['\n', '\r'];
        let newline = filter(|c| newlines.contains(c));
        let text = filter(|c| !operators.contains(c) && !newlines.contains(c))
            .repeated()
            .at_least(1)
            .collect::<String>();

        // let link_text = just('[').ignore_then(expr.clone()).then_ignore(just(']'));
        // let link_url = just('(')
        //     .ignore_then(filter(|c| *c != ')').repeated())
        //     .then_ignore(just(')'))
        //     .collect::<String>();
        let link_text = expr.clone().delimited_by(just('['), just(']'));
        let link_url = text.delimited_by(just('('), just(')'));

        let link = link_text.then(link_url).map(|(text, url)| Expr::Link {
            text: Box::new(text),
            url,
        });

        title
            .or(link)
            .or(text.map(Expr::Text))
            .or(newline.map(|_| Expr::Newline))
    });
    // start
    //     .ignore_then(expr.repeated())
    //     .map(Expr::Exprs)
    //     .then_ignore(just("\"#"))
    expr.repeated().map(Expr::Exprs).then_ignore(end())
}

impl Expr {
    fn eval(&self) -> TokenStream {
        match self {
            Expr::Title(t) => {
                let title = format!("<h1>{}</h1>", t.eval().to_string());
                quote! {#title}.into()
            }
            Expr::Text(t) => quote! {
                #t
            }
            .into(),
            Expr::Link { text, url } => {
                let text = text.eval().to_string();
                quote! {
                    <a href=#url>{#text}</a>
                }
            }
            .into(),
            Expr::Exprs(exprs) => exprs.iter().map(Expr::eval).collect(),
            _ => quote! {}.into(),
        }
    }
}

#[derive(Debug)]
enum Expr {
    Title(Box<Expr>),
    Text(String),
    Exprs(Vec<Expr>),
    Link { text: Box<Expr>, url: String },
    Newline,
}
