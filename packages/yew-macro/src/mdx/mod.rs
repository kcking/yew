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

    parsed
}

fn parse_mdx() -> impl Parser<char, Expr, Error = Simple<char>> {
    let expr = recursive(|expr| {
        let title = just('#')
            .padded()
            .ignore_then(expr.clone())
            .map(|t| Expr::Title(Box::new(t)));

        let operators = &['(', ')', '[', ']'];
        let newlines = &['\n', '\r'];
        let newline = filter(|c| newlines.contains(c));
        let text = filter(|c| !operators.contains(c) && !newlines.contains(c))
            .repeated()
            .at_least(1)
            .collect::<String>();

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
    expr.repeated().map(Expr::Exprs).then_ignore(end())
}

impl Expr {
    fn eval(&self) -> TokenStream {
        match self {
            Expr::Title(t) => format!("<h1>{{{}}}</h1>", t.eval()).parse().unwrap(),
            Expr::Text(t) => format!("{{\"{}\"}}", t).parse().unwrap(),
            Expr::Link { text, url } => {
                let text = text.eval();
                format!("<a href=\"{url}\">{text}</a>").parse().unwrap()
            }
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
